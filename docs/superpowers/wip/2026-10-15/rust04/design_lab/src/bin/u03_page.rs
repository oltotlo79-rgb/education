use std::fmt::Display;

// どんな型の一覧にも使える「1ページ分」の入れ物
#[derive(Debug)]
struct Page<T> {
    items: Vec<T>,
    page: u32,
    per_page: u32,
    total: usize,
}

// すべての T で使えるメソッド
impl<T> Page<T> {
    fn has_next(&self) -> bool {
        (self.page as usize) * (self.per_page as usize) < self.total
    }
}

// T が Display を実装しているときだけ使えるメソッド
impl<T: Display> Page<T> {
    fn render(&self) -> String {
        let lines: Vec<String> = self.items.iter().map(|x| format!("- {x}")).collect();
        format!(
            "{}\n({}ページ目 / 全{}件)",
            lines.join("\n"),
            self.page,
            self.total
        )
    }
}

// 何かの並び（Vec でも配列でもイテレータでもよい）から、1ページ分を切り出す
fn paginate<T, I>(all: I, page: u32, per_page: u32) -> Page<T>
where
    I: IntoIterator<Item = T>,
{
    let all: Vec<T> = all.into_iter().collect();
    let total = all.len();
    let skip = ((page - 1) * per_page) as usize;
    let items = all.into_iter().skip(skip).take(per_page as usize).collect();
    Page {
        items,
        page,
        per_page,
        total,
    }
}

fn main() {
    let titles = vec!["牛乳", "本", "電気代", "歯医者", "散髪"];
    let p1 = paginate(titles.clone(), 1, 2);
    println!("{}", p1.render());
    println!("次のページはある？ {}", p1.has_next());

    let p3 = paginate(titles, 3, 2);
    println!("{}", p3.render());
    println!("次のページはある？ {}", p3.has_next());

    // 数字の範囲（1..=7）からも同じ関数で切り出せる
    let numbers = paginate(1..=7, 2, 3);
    println!("{numbers:?}");
}
