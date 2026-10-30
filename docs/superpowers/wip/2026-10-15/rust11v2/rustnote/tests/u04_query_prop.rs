//! 第11巻 UNIT 04: NoteQuery の offset（何件読み飛ばすか）を、どんなページ番号でも確かめる。
//!
//! 実行: cargo test --test u04_query_prop

use proptest::prelude::*;
use rustnote::domain::{NoteQuery, PER_PAGE};

proptest! {
    // URL の ?page= から作るとき（NoteQuery::new）
    #[test]
    fn offset_from_new(page in any::<Option<u32>>()) {
        let query = NoteQuery::new(None, page);
        prop_assert!(query.page >= 1);
        prop_assert_eq!(query.offset(), u64::from(query.page - 1) * u64::from(PER_PAGE));
    }

    // 項目を直接書いて作るとき（項目は pub なので、だれでもこう書ける）
    #[test]
    fn offset_from_fields(page in any::<u32>()) {
        let query = NoteQuery { keyword: None, page };
        prop_assert!(query.offset() < u64::from(page) * u64::from(PER_PAGE));
    }
}
