// 結果は display_name から作るか、email から作る。どちらも同じ 'a
fn display_name<'a>(nickname: Option<&'a str>, email: &'a str) -> &'a str {
    match nickname {
        Some(name) if !name.trim().is_empty() => name,
        _ => email,
    }
}

// 結果は text の一部分だけ。prefix とは無関係なので prefix に 'a は付けない
fn after_prefix<'a>(text: &'a str, prefix: &str) -> &'a str {
    text.strip_prefix(prefix).unwrap_or(text)
}

fn main() {
    let email = String::from("alice@example.com");
    println!("{}", display_name(Some("アリス"), &email));
    println!("{}", display_name(Some("  "), &email));
    println!("{}", display_name(None, &email));

    let path = String::from("/notes/42");
    let rest;
    {
        let prefix = String::from("/notes/");
        rest = after_prefix(&path, &prefix);
        // ここで prefix は片付けられる
    }
    // rest は path だけを借りているので、まだ使える
    println!("id 部分 = {rest}");
}
