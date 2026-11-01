//! 設定を、環境変数から読む。名前は onprem の 04 のサービスの XML（WinSW）と同じ
//!
//! | 名前 | 意味 | 開発のパソコンで無いとき |
//! |---|---|---|
//! | `APP_ENV` | development / staging / production | development |
//! | `APP_BASE_URL` | メールに書く URL の頭 | http://localhost:3000 |
//! | `DATABASE_URL` | PostgreSQL の接続先 | （必ず要る） |
//! | `FILE_DIR` | 添付ファイルを置くフォルダ | ./files |
//! | `SMTP_HOST` / `SMTP_PORT` | メールの中継先（M365 の SMTP リレー）と、ポート | 無ければ ./sent_mail にファイルで書く |
//! | `SMTP_USE_TLS` | STARTTLS で暗号化するか（false は開発のパソコンだけ。09 の練習用の SMTP サーバー向け） | true |
//! | `MAIL_FROM` | 送り主のアドレス | noreply@example.com |
//! | `MAIL_REDIRECT_TO` | （検証だけ）全部のメールを、この宛先に送る | 無し |
//!
//! `HOST` と `PORT`（待ち受ける所）は、Topcoat の `start` が自分で読む。

use std::{env, fmt, path::PathBuf};

/// どこで動いているか
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEnv {
    Development,
    Staging,
    Production,
}

/// メールの出し方
#[derive(Debug, Clone)]
pub enum MailSettings {
    /// 開発のパソコン: 送らずに、.eml のファイルとしてフォルダに書く
    File(PathBuf),
    /// サーバー: SMTP リレーに渡す（starttls が true なら、STARTTLS で暗号化してから渡す）
    Smtp { host: String, port: u16, starttls: bool },
}

#[derive(Debug, Clone)]
pub struct Config {
    pub app_env: AppEnv,
    pub base_url: String,
    pub database_url: String,
    pub file_dir: PathBuf,
    pub mail: MailSettings,
    pub mail_from: String,
    pub mail_redirect_to: Option<String>,
}

/// 設定の間違い。起動のときに、すぐ止まって知らせる
#[derive(Debug)]
pub struct ConfigError(String);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "設定の間違い: {}", self.0)
    }
}

impl std::error::Error for ConfigError {}

/// 空の値は「無い」と同じに扱う
fn var(name: &str) -> Option<String> {
    env::var(name).ok().map(|v| v.trim().to_string()).filter(|v| !v.is_empty())
}

impl Config {
    pub fn from_env() -> Result<Config, ConfigError> {
        let app_env = match var("APP_ENV").as_deref() {
            None | Some("development") => AppEnv::Development,
            Some("staging") => AppEnv::Staging,
            Some("production") => AppEnv::Production,
            Some(other) => return Err(ConfigError(format!("APP_ENV は development・staging・production のどれかです（今: {other}）"))),
        };
        let dev = app_env == AppEnv::Development;

        // サーバーでは、決まりの値に頼らない（書き忘れに、すぐ気付けるように）
        let required = |name: &str, dev_default: &str| -> Result<String, ConfigError> {
            match var(name) {
                Some(v) => Ok(v),
                None if dev => Ok(dev_default.to_string()),
                None => Err(ConfigError(format!("{name} がありません"))),
            }
        };

        let database_url = var("DATABASE_URL").ok_or_else(|| ConfigError("DATABASE_URL がありません".into()))?;
        let base_url = required("APP_BASE_URL", "http://localhost:3000")?.trim_end_matches('/').to_string();
        let file_dir = PathBuf::from(required("FILE_DIR", "files")?);
        let mail_from = required("MAIL_FROM", "noreply@example.com")?;

        let mail = match var("SMTP_HOST") {
            Some(host) => {
                let port = var("SMTP_PORT").unwrap_or_else(|| "25".into());
                let port = port.parse().map_err(|_| ConfigError(format!("SMTP_PORT が数ではありません（今: {port}）")))?;
                let starttls = match var("SMTP_USE_TLS").as_deref() {
                    None | Some("true") => true,
                    Some("false") => false,
                    Some(other) => return Err(ConfigError(format!("SMTP_USE_TLS は true か false です（今: {other}）"))),
                };
                // 暗号化しないで送るのは、開発のパソコンの中の練習だけ（会社のネットワークに平文で流さない）
                if !starttls && !dev {
                    return Err(ConfigError("SMTP_USE_TLS=false は、APP_ENV=development のときだけ使えます".into()));
                }
                MailSettings::Smtp { host, port, starttls }
            }
            None if dev => MailSettings::File(PathBuf::from("sent_mail")),
            None => return Err(ConfigError("SMTP_HOST がありません".into())),
        };

        let mail_redirect_to = var("MAIL_REDIRECT_TO");
        // 検証のサーバーから、本物の宛先にメールが届く事故を防ぐ
        if app_env == AppEnv::Staging && mail_redirect_to.is_none() {
            return Err(ConfigError("APP_ENV=staging のときは MAIL_REDIRECT_TO が要ります".into()));
        }

        Ok(Config { app_env, base_url, database_url, file_dir, mail, mail_from, mail_redirect_to })
    }
}
