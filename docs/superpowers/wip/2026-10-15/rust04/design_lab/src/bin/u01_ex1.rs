mod paging {
    // 1 から始まるページ番号。0 ページは存在しない
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PageNumber(u32);

    impl PageNumber {
        pub fn new(n: u32) -> Option<PageNumber> {
            if n == 0 { None } else { Some(PageNumber(n)) }
        }

        // 1 ページ目なら 0 件飛ばす、2 ページ目なら per_page 件飛ばす
        pub fn offset(self, per_page: u32) -> u32 {
            (self.0 - 1) * per_page
        }
    }
}

use paging::PageNumber;

fn main() {
    println!("{:?}", PageNumber::new(0));
    let page = PageNumber::new(3).unwrap();
    println!("{:?} の offset = {}", page, page.offset(20));
}
