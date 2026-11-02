//! exe のコマンド。1 つの webapp.exe が、サーバーと、管理用のコマンドを兼ねる
//!
//! ```text
//! webapp.exe                                   サーバーとして動く（serve と同じ）
//! webapp.exe import-employees 社員.csv [--dry-run]   社員の CSV を取り込む
//! webapp.exe grant-role --employee-no 090001 --role admin   役割を変える（member・worker・admin）
//! webapp.exe set-password --employee-no 012345  パスワードを決める（標準入力から 1 行読む）
//! webapp.exe send-daily-reminders               毎朝の通知を、今すぐ 1 回動かす
//! webapp.exe send-test-mail --to 自分のアドレス   テストのメールを 1 通送る（メールの道を確かめる。09）
//! webapp.exe seed-demo                          練習用の社員を作る（開発のパソコンだけ）
//! ```

use std::{io::BufRead, sync::Arc};

use anyhow::{Context, bail};
use topcoat::context::{AppContext, Cx};

use crate::{
    app, auth,
    config::{AppEnv, Config},
    employees, jobs,
    mail::mail_config,
    models::Role,
    state::{AppState, connect, migrate},
    workflow,
};

// 行末の \ は「次の行へ続く（改行と、次の行の頭の空白は入らない）」という印
const USAGE: &str = "使い方: webapp [serve | import-employees <CSV> [--dry-run] \
                     | grant-role --employee-no <番号> --role <member|worker|admin> \
                     | set-password --employee-no <番号> | send-daily-reminders \
                     | send-test-mail --to <アドレス> | seed-demo]";

pub async fn run(args: Vec<String>) -> anyhow::Result<()> {
    // 開発のパソコンでは、.env のファイルから環境変数を読む（サーバーには .env を置かず、サービスの XML に書く）
    let _ = dotenvy::dotenv();
    let config = Config::from_env()?;
    let command = args.first().map(String::as_str).unwrap_or("serve");
    let rest = args.get(1..).unwrap_or_default();

    match command {
        "serve" => serve(config).await,
        "import-employees" => import_employees(config, rest).await,
        "grant-role" => {
            let no = option(rest, "--employee-no")?;
            let role = Role::try_from(option(rest, "--role")?).map_err(anyhow::Error::msg)?;
            let state = open(config).await?;
            workflow::grant_role(&state.pool, &no, role).await?;
            println!("{no} の役割を「{}」にしました。", role.label());
            Ok(())
        }
        "set-password" => set_password(config, rest).await,
        "send-daily-reminders" => {
            let state = open(config).await?;
            let cx = Cx::new(background_context(&state)?);
            let r = jobs::daily_reminders(&cx).await?;
            println!("送り直した承認のお願い: {} 件 / 納期が近い依頼: {} 件", r.resent, r.due_soon);
            Ok(())
        }
        "send-test-mail" => send_test_mail(config, rest).await,
        "seed-demo" => seed_demo(config).await,
        "help" | "--help" | "-h" => {
            println!("{USAGE}");
            Ok(())
        }
        other => bail!("知らないコマンドです: {other}\n{USAGE}"),
    }
}

/// `--name 値` の形の値を取り出す
fn option(args: &[String], name: &str) -> anyhow::Result<String> {
    let i = args.iter().position(|a| a == name).with_context(|| format!("{name} がありません\n{USAGE}"))?;
    args.get(i + 1).cloned().with_context(|| format!("{name} の値がありません"))
}

/// データベースにつなぎ、表を新しくする（まだ流していないマイグレーションを流す）
async fn open(config: Config) -> anyhow::Result<AppState> {
    let pool = connect(&config.database_url).await.context("データベースにつなげません（DATABASE_URL を確かめる）")?;
    migrate(&pool).await.context("マイグレーションに失敗しました")?;
    Ok(AppState { pool, config: Arc::new(config) })
}

/// 画面のリクエストの外（毎朝の処理・コマンド）で使う、アプリの共有の物の入れ物
fn background_context(state: &AppState) -> anyhow::Result<Arc<AppContext>> {
    let mut app = AppContext::new();
    app.insert(state.clone());
    app.insert(mail_config(&state.config.mail)?);
    Ok(Arc::new(app))
}

async fn serve(config: Config) -> anyhow::Result<()> {
    let state = open(config).await?;
    let mail = mail_config(&state.config.mail)?;
    jobs::spawn_daily(background_context(&state)?);
    println!("起動します（{:?}）。メールの URL は {} で始まります。", state.config.app_env, state.config.base_url);
    // HOST と PORT（待ち受ける所）は、Topcoat が環境変数から読む。無ければ 127.0.0.1:3000
    topcoat::start(app::router(state, mail)).await?;
    Ok(())
}

async fn import_employees(config: Config, args: &[String]) -> anyhow::Result<()> {
    let path = args.iter().find(|a| !a.starts_with("--")).with_context(|| format!("CSV のファイルを指定してください\n{USAGE}"))?;
    let dry_run = args.iter().any(|a| a == "--dry-run");
    let bytes = std::fs::read(path).with_context(|| format!("{path} を読めません"))?;
    let rows = match employees::read_csv(&bytes) {
        Ok(rows) => rows,
        Err(errors) => {
            for e in &errors {
                eprintln!("× {e}");
            }
            bail!("{} 件の間違いがあるので、取り込みませんでした。", errors.len());
        }
    };
    if dry_run {
        println!("間違いはありません（{} 件）。--dry-run なので、取り込んでいません。", rows.len());
        return Ok(());
    }
    let state = open(config).await?;
    let (created, updated) = employees::import(&state.pool, &rows).await?;
    println!("取り込みました（新しく {created} 件・書きかえ {updated} 件）。");
    Ok(())
}

/// テストのメールを 1 通送る。データベースは使わないので、つながなくても動く
async fn send_test_mail(config: Config, args: &[String]) -> anyhow::Result<()> {
    let to = option(args, "--to")?;
    let where_to = match &config.mail {
        crate::config::MailSettings::File(dir) => format!("ファイル（{}）", dir.display()),
        crate::config::MailSettings::Smtp { host, port, starttls } => {
            format!("{host}:{port}（STARTTLS: {}）", if *starttls { "使う" } else { "使わない" })
        }
    };
    let now = time::OffsetDateTime::now_utc().to_offset(crate::models::JST);
    let body = format!(
        "依頼・承認アプリからの、送信テストです。\n\n送った時刻: {}\nAPP_ENV: {:?}\n渡した先: {where_to}\n",
        crate::models::fmt_datetime(now),
        config.app_env
    );
    // connect_lazy は、本当に使うまでデータベースにつながない
    let pool = sqlx::PgPool::connect_lazy(&config.database_url)?;
    let state = AppState { pool, config: Arc::new(config) };
    let cx = Cx::new(background_context(&state)?);
    if let Err(e) = crate::mail::send_now(&cx, crate::mail::Outgoing::new(&to, "送信テスト", body)).await {
        // 原因の連なり（「つなげない」←「証明書を確かめられない」など）を、全部書き出す
        let mut message = e.to_string();
        let mut source = e.source();
        while let Some(s) = source {
            message.push_str(&format!("\n  原因: {s}"));
            source = s.source();
        }
        bail!("送れませんでした（渡した先: {where_to}）: {message}");
    }
    println!("渡しました（宛先: {to}・渡した先: {where_to}）。届いたかを、受け取る側で確かめてください。");
    Ok(())
}

async fn set_password(config: Config, args: &[String]) -> anyhow::Result<()> {
    let no = option(args, "--employee-no")?;
    eprintln!("{no} の新しいパスワードを入れて、Enter を押してください（12 文字以上）:");
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line)?;
    let password = line.trim_end_matches(['\r', '\n']).to_string();
    if password.chars().count() < 12 {
        bail!("パスワードは 12 文字以上にしてください。");
    }
    let state = open(config).await?;
    let hash = auth::hash_password(password).await;
    let n = sqlx::query("UPDATE employees SET password_hash = $2 WHERE employee_no = $1")
        .bind(&no)
        .bind(hash)
        .execute(&state.pool)
        .await?
        .rows_affected();
    if n == 0 {
        bail!("従業員番号 {no} の社員がいません。");
    }
    println!("{no} のパスワードを決めました。");
    Ok(())
}

/// 練習用の社員（06 の seed_demo と同じ 5 人）
pub const DEMO_PEOPLE: [(&str, &str, &str, &str, Role); 5] = [
    ("012345", "山田 太郎", "設計部", "主任", Role::Member),
    ("012346", "佐藤 花子", "設計部", "課長", Role::Member),
    ("012347", "鈴木 一郎", "設計部", "部長", Role::Member),
    ("020001", "高橋 三郎", "設備課", "係長", Role::Worker),
    ("090001", "管理 次郎", "情報システム部", "", Role::Admin),
];
pub const DEMO_PASSWORD: &str = "practice-pass-2026"; // 練習用。本物のパスワードには使わない

async fn seed_demo(config: Config) -> anyhow::Result<()> {
    if config.app_env != AppEnv::Development {
        bail!("APP_ENV が development のとき（開発のパソコン）だけ使えます。");
    }
    let state = open(config).await?;
    for (no, name, dept, title, role) in DEMO_PEOPLE {
        // ハッシュは 1 人ずつ作る。塩（salt）が毎回違うので、同じパスワードでも、ハッシュは人ごとに違う
        // （1 回作って使い回すと、全員のハッシュが同じになり、「同じパスワードの人」が見えてしまう）
        let hash = auth::hash_password(DEMO_PASSWORD.to_string()).await;
        sqlx::query(
            "INSERT INTO employees (employee_no, name, dept, job_title, email, role, password_hash)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             ON CONFLICT (employee_no) DO UPDATE
                SET name = EXCLUDED.name, dept = EXCLUDED.dept, job_title = EXCLUDED.job_title,
                    email = EXCLUDED.email, role = EXCLUDED.role, password_hash = EXCLUDED.password_hash, is_active = TRUE",
        )
        .bind(no)
        .bind(name)
        .bind(dept)
        .bind(title)
        .bind(format!("{no}@example.com"))
        .bind(role.as_str())
        .bind(&hash)
        .execute(&state.pool)
        .await?;
    }
    println!("{} 人を作りました。パスワードは全員 {DEMO_PASSWORD} です（練習用）。", DEMO_PEOPLE.len());
    Ok(())
}
