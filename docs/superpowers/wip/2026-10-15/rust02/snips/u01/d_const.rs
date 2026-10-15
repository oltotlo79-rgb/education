// 定数はファイルのどこからでも使える。型の注釈（: usize など）が必須
const MAX_TITLE_CHARS: usize = 80;
// 定数の値は、コンパイルするときに計算できる式なら書ける
const SECONDS_PER_DAY: u64 = 24 * 60 * 60;
const SESSION_DAYS: u64 = 30;

fn main() {
    println!("タイトルは最大 {MAX_TITLE_CHARS} 文字");

    let session_seconds = SESSION_DAYS * SECONDS_PER_DAY;
    println!("セッションの有効期限: {session_seconds} 秒");

    // 数字の中の _ は読みやすくするための区切りで、値は変わらない
    let big = 1_000_000;
    println!("百万: {big}");
}
