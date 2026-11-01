//! メールを送る。送り方（ファイル・SMTP リレー）は設定で選び、送る側のコードは変えない
//!
//! 大事な決まり:
//! 1. **データベースへの保存が確定してから送る。** 処理の途中では「送る物の箱（Outbox）」に入れておき、
//!    `commit` が成功したあとで `send_all` する。途中で失敗して取り消したら、箱ごと捨てる（送らない）。
//! 2. **検証のサーバーでは、本物の宛先に送らない。** `MAIL_REDIRECT_TO` があれば、全部そこへ送る。

use topcoat::{
    context::Cx,
    mail::{FileTransport, MailConfig, SmtpTransport, mail, send},
};

use crate::{
    config::{Config, MailSettings},
    state::state,
};

/// 件名の頭に付ける印（メールの一覧で、このアプリからのメールだと分かるように）
const SUBJECT_PREFIX: &str = "【依頼・承認】";

/// 設定から、Topcoat のメールの設定を作る（起動のときに 1 回）
pub fn mail_config(settings: &MailSettings) -> Result<MailConfig, topcoat::mail::SmtpError> {
    let builder = MailConfig::builder();
    let builder = match settings {
        // 開発のパソコン: 送らずに、フォルダに .eml のファイルとして書く（メールソフトで開ける）
        MailSettings::File(dir) => builder.transport(FileTransport::new(dir)),
        // サーバー: M365 の SMTP リレーへ。STARTTLS で暗号化してから渡す（ポートは 25）
        MailSettings::Smtp { host, port } => builder.transport(SmtpTransport::starttls(host)?.port(*port).build()),
    };
    Ok(builder.build())
}

/// 送る前のメール 1 通
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outgoing {
    pub to: String,
    pub subject: String,
    pub body: String,
}

/// 送る物の箱。処理の途中で入れておき、保存が確定してから、まとめて送る
#[derive(Debug, Default)]
pub struct Outbox {
    items: Vec<Outgoing>,
}

impl Outbox {
    pub fn new() -> Outbox {
        Outbox::default()
    }

    pub fn add(&mut self, to: &str, subject: &str, body: String) {
        self.items.push(Outgoing { to: to.to_string(), subject: format!("{SUBJECT_PREFIX}{subject}"), body });
    }

    pub fn items(&self) -> &[Outgoing] {
        &self.items
    }

    /// まとめて送る。1 通が失敗しても、ほかは送る（失敗はログに書く）
    ///
    /// 保存はもう確定しているので、ここでエラーを返して画面をエラーにはしない。
    /// 届かなかった承認のお願いは、管理者の「送り直す」か、毎朝の処理で送り直せる。
    pub async fn send_all(self, cx: &Cx) {
        let config = &state(cx).config;
        for item in self.items {
            let item = redirect(config, item);
            if let Err(e) = send_one(cx, &config.mail_from, &item).await {
                eprintln!("メールを送れませんでした（宛先: {}・件名: {}）: {e}", item.to, item.subject);
            }
        }
    }
}

/// 検証のサーバーでは、宛先を差し替える。本来の宛先は本文の先頭に書く
pub fn redirect(config: &Config, item: Outgoing) -> Outgoing {
    match &config.mail_redirect_to {
        None => item,
        Some(to) => Outgoing {
            to: to.clone(),
            subject: format!("【検証】{}", item.subject),
            body: format!("（検証のメールです。本来の宛先: {}）\n\n{}", item.to, item.body),
        },
    }
}

async fn send_one(cx: &Cx, from: &str, item: &Outgoing) -> topcoat::Result<()> {
    let mail = mail! {
        from: from,
        to: item.to.as_str(),
        subject: item.subject.as_str(),
        text: item.body.as_str(),
    }?;
    send(cx, mail).await?;
    Ok(())
}
