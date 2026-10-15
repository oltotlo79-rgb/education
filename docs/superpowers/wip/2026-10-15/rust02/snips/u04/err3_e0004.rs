fn main() {
    let code: u16 = 404;
    let label = match code {
        200..=299 => "成功",
        400..=499 => "リクエストの誤り",
        500..=599 => "サーバーの誤り",
    };
    println!("{label}");
}
