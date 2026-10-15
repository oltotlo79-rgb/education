const MAX_TITLE_CHARS: usize = 80;

fn main() {
    // フォームから届いたタイトル（前後に空白がついている）
    let title = "   牛乳を買う   ";
    // 前後の空白を取り除いた版を、同じ名前で作り直す（シャドーイング）
    let title = title.trim();
    // 文字数を数える（日本語も 1 文字を 1 と数える方法。UNIT 05 で詳しく）
    let chars = title.chars().count();
    println!("タイトル「{title}」は {chars} 文字です（上限 {MAX_TITLE_CHARS} 文字）");
}
