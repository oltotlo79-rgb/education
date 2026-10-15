fn main() {
    // 文字列リテラルは &str 型（プログラムに埋め込まれた、書き換えられない文字列）
    let greeting: &str = "こんにちは";

    // String::from で、書き足せる String を作る
    let mut message = String::from(greeting);
    // push_str で文字列を後ろに足す
    message.push_str("、Rust");
    // push で1文字（char）を足す
    message.push('！');
    println!("{message}");

    // to_string でも String を作れる
    let name = "さくら".to_string();
    // format! は println! と同じ書き方で、表示せずに String を作る
    let line = format!("{name}さん、{message}");
    println!("{line}");

    // + でつなぐこともできる（左側の String は使い切られる）
    let a = String::from("Rust");
    let b = a + "入門";
    println!("{b}");
}
