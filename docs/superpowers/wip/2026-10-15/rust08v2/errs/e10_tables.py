# 第8巻 UNIT 10 で、RustNote のコピーに当てる書き換え（edit_src.py が REPLACEMENTS を読む）。
# 使い方: e10_load.py の引数で、どの表を使うかを選ぶ。

# 監査の記録を足す（lib.rs）
AUDIT_LIB = [("pub mod auth;\n", "pub mod audit;\npub mod auth;\n")]

# 監査の記録を足す（login.rs）
AUDIT_LOGIN = [
    (
        "use crate::auth::{authenticate, current_user, log_in};\n",
        "use crate::audit::{self, Event};\nuse crate::auth::{authenticate, current_user, log_in};\n",
    ),
    (
        """        log_in(cx, &user).await?;
""",
        """        log_in(cx, &user).await?;
        audit::record(cx, Event::LoginSucceeded(user.id));
""",
    ),
    (
        """    // メールが無いのか、パスワードが違うのかは区別しない
""",
        """    audit::record(cx, Event::LoginFailed(&input.email));
    // メールが無いのか、パスワードが違うのかは区別しない
""",
    ),
]

# 監査の記録を足す（logout.rs）
AUDIT_LOGOUT = [
    (
        "use crate::auth::log_out;\n",
        "use crate::audit::{self, Event};\nuse crate::auth::{current_user, log_out};\n",
    ),
    (
        """    log_out(cx).await?;
""",
        """    let user = current_user(cx).await?.map(|user| user.id);
    log_out(cx).await?;
    audit::record(cx, Event::LoggedOut(user));
""",
    ),
]

# エラー2: 監査の記録に、フォームをまるごと書く（Debug が無いので E0277）
FORM_LOG = [
    (
        """    audit::record(cx, Event::LoginFailed(&input.email));
""",
        """    eprintln!("[audit] login_failed {input:?}");
""",
    ),
]

# エラー3: Debug を足して、コンパイルを通す（パスワードが記録に出る）
FORM_DEBUG = [("#[derive(Deserialize)]\npub struct LoginForm", "#[derive(Debug, Deserialize)]\npub struct LoginForm")]
