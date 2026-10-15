fn main() {
    let url = "http://127.0.0.1:8080/notes/42/edit?from=list";

    let (scheme, rest) = url.split_once("://").unwrap();
    let (host_and_port, path_and_query) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };

    // 「ホスト:ポート」を「:」で分ける。ポートが書かれていなければ既定値を使う
    let (host, port) = match host_and_port.split_once(':') {
        Some((host, port)) => (host, port),
        None if scheme == "https" => (host_and_port, "443"),
        None => (host_and_port, "80"),
    };

    let (path, query) = path_and_query
        .split_once('?')
        .unwrap_or((path_and_query, ""));

    println!("スキーム: {scheme}");
    println!("ホスト  : {host}");
    println!("ポート  : {port}");
    println!("パス    : {path}");
    println!("クエリ  : {query}");
}
