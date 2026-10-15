fn main() {
    for raw in [8080_i64, 70000, -1] {
        match u16::try_from(raw) {
            Ok(port) => println!("{raw}: ポート {port} を使います"),
            Err(e) => println!("{raw}: 使えません（{e}）"),
        }
    }
}
