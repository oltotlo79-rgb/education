// 摂氏（℃）を華氏（℉）に変換する
fn fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    println!("0℃ = {}℉", fahrenheit(0.0));
    println!("36.5℃ = {:.1}℉", fahrenheit(36.5));
    println!("100℃ = {}℉", fahrenheit(100.0));
}
