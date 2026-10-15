// 本文の「どこか1行」を借りて持つ struct
// 'a は「この struct は、借りている本文より長生きしない」という印
#[derive(Debug)]
struct Highlight<'a> {
    line_no: usize,
    text: &'a str,
}

impl<'a> Highlight<'a> {
    // 返す &str は、struct ではなく元の本文と同じ寿命
    fn text(&self) -> &'a str {
        self.text
    }
}

// 見つかった行を借りたまま返す。新しい String は作らない
fn find_line<'a>(body: &'a str, keyword: &str) -> Option<Highlight<'a>> {
    for (i, line) in body.lines().enumerate() {
        if line.contains(keyword) {
            return Some(Highlight {
                line_no: i + 1,
                text: line,
            });
        }
    }
    None
}

fn main() {
    let body = String::from("牛乳を買う\n郵便局へ行く\n本を返す");
    match find_line(&body, "郵便") {
        Some(h) => println!("{}行目: {} / {:?}", h.line_no, h.text(), h),
        None => println!("見つかりません"),
    }
}
