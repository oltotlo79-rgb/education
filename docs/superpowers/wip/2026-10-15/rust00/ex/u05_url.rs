fn main() {
    let url = "https://your-app.fly.dev/notes?q=rust&page=2";
    println!("URL 全体: {url}");

    // 「://」の前がスキーム（通信のやり方）
    let (scheme, rest) = url.split_once("://").unwrap();
    println!("スキーム: {scheme}");

    // 最初の「/」の前がホスト（どのコンピュータか）
    let (host, path_and_query) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };
    println!("ホスト  : {host}");

    // 「?」の前がパス（どのページか）、後ろがクエリ（追加の条件）
    let (path, query) = path_and_query
        .split_once('?')
        .unwrap_or((path_and_query, ""));
    println!("パス    : {path}");

    // クエリは「名前=値」が & でつながっている
    for pair in query.split('&') {
        if let Some((name, value)) = pair.split_once('=') {
            println!("クエリ  : {name} = {value}");
        }
    }
}
