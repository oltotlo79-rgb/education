// views/src/lib.rs: トップページへのリンクを、app の関数で作る
pub fn back_link() -> String {
    format!("<a href=\"{}\">トップページへ戻る</a>", app::home_path())
}
