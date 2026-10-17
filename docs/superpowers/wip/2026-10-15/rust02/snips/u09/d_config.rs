// 設定の読み込みで起こりうる失敗
#[derive(Debug)]
enum ConfigError {
    Missing,
    NotANumber(String),
    OutOfRange(u32),
}

// ポート番号の文字列（ないかもしれない）を検証して u16 にする
fn parse_port(raw: Option<&str>) -> Result<u16, ConfigError> {
    // ok_or: None を Err(...) に変える。? で、なければすぐ返す
    let text = raw.ok_or(ConfigError::Missing)?;
    // map_err: ParseIntError を、自分のエラー型 ConfigError に変える
    let value: u32 = text
        .trim()
        .parse()
        .map_err(|_| ConfigError::NotANumber(text.to_string()))?;
    // u16 に入らない値（65536 以上）を弾く
    u16::try_from(value).map_err(|_| ConfigError::OutOfRange(value))
}

fn main() {
    for raw in [Some("8080"), None, Some("abc"), Some("70000")] {
        // エラーの種類ごとに、中のデータを使ってメッセージを作る
        match parse_port(raw) {
            Ok(port) => println!("{raw:?} → ポート {port}"),
            Err(ConfigError::Missing) => println!("{raw:?} → 設定がありません"),
            Err(ConfigError::NotANumber(text)) => {
                println!("{raw:?} → 「{text}」は数ではありません")
            }
            Err(ConfigError::OutOfRange(n)) => {
                println!("{raw:?} → {n} はポート番号の範囲外です")
            }
        }
    }
}
