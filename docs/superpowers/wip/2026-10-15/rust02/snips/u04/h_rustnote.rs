const PER_PAGE: u32 = 20;

fn main() {
    // ノートが全部で 45 件あるとする
    let total_notes: u32 = 45;
    // 必要なページ数（端数は切り上げ。45 ÷ 20 → 3）
    let total_pages = total_notes.div_ceil(PER_PAGE);

    for page in 1..=total_pages {
        let has_prev = page > 1;
        let has_next = page < total_pages;
        // 2つの bool の組み合わせで、ページ送りの表示を決める
        let nav = match (has_prev, has_next) {
            (false, false) => "（ページ送りなし）",
            (false, true) => "次へ →",
            (true, true) => "← 前へ | 次へ →",
            (true, false) => "← 前へ",
        };
        println!("{page}/{total_pages} ページ: {nav}");
    }
}
