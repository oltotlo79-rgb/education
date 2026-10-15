// 1ページに表示するノートの件数
const PER_PAGE: u32 = 20;

fn main() {
    // URL の ?page= に入ってくる文字列をいくつか試す
    for raw in ["1", "3", "0", "-2", "abc"] {
        // 数（u32）にできなければ 1 ページ目とみなす
        let page: u32 = raw.parse().unwrap_or(1);
        // 0 ページ目は存在しないので、最低でも 1 にそろえる
        let page = page.max(1);
        // 先頭から何件を飛ばして表示するか
        let offset = (page - 1) * PER_PAGE;
        println!("?page={raw} → {page} ページ目, {offset} 件飛ばす");
    }
}
