//! 第13巻 UNIT 04 エラー3: AppError を返す関数の中で、数の変換の失敗に ? を付けた。
//! 実行: cargo run -q --example e04_question

use rustnote::error::AppError;

fn parse_page(input: &str) -> Result<u32, AppError> {
    let page: u32 = input.parse()?;
    Ok(page)
}

fn main() {
    println!("{:?}", parse_page("3").map_err(|error| error.to_string()));
}
