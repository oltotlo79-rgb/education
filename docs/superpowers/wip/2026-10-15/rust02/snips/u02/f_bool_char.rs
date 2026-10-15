fn main() {
    let logged_in = true;
    let is_owner = false;
    // && は「かつ」、|| は「または」、! は「ではない」
    println!("編集できる: {}", logged_in && is_owner);
    println!("閲覧できる: {}", logged_in || is_owner);
    println!("ログアウト中: {}", !logged_in);

    // char は Unicode の1文字。番号（コードポイント）に変換できる
    let c = 'あ';
    println!("'{c}' の番号: {} (16進数で {:X})", c as u32, c as u32);
    // 番号から char に戻す（存在しない番号もあるので、結果は Some か None）
    println!("0x41 は {:?}", char::from_u32(0x41));
    // 数字の文字を数に変換する
    println!("'7' を数に: {:?}", '7'.to_digit(10));
    // 文字の種類を調べる
    println!("'A' は英字? {}", 'A'.is_alphabetic());
    println!("'あ' は ASCII の英数字? {}", 'あ'.is_ascii_alphanumeric());
}
