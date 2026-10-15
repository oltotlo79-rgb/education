// 規則1・2: 入力の参照が1つだけなら、出力はその入力と同じ寿命になる
// 省略せずに書くと fn trim_title<'a>(s: &'a str) -> &'a str
fn trim_title(s: &str) -> &str {
    s.trim()
}

struct Notebook {
    titles: Vec<String>,
}

impl Notebook {
    // 規則3: &self があれば、出力は self と同じ寿命になる
    // 省略せずに書くと fn find<'a, 'b>(&'a self, keyword: &'b str) -> Option<&'a str>
    fn find(&self, keyword: &str) -> Option<&str> {
        for title in &self.titles {
            if title.contains(keyword) {
                return Some(title.as_str());
            }
        }
        None
    }
}

fn main() {
    let raw = String::from("   週報を書く   ");
    println!("[{}]", trim_title(&raw));

    let book = Notebook {
        titles: vec![String::from("買い物リスト"), String::from("掃除の当番")],
    };
    let found;
    {
        // 検索語はすぐ片付けられるが、結果は book を借りているだけなので問題ない
        let keyword = String::from("掃除");
        found = book.find(&keyword);
    }
    println!("{found:?}");
    println!("{:?}", book.find("旅行"));
}
