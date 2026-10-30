//! 第11巻 UNIT 04: NoteQuery を、項目を直接書いて作り（page は pub）、offset がパニックしないかを試す。
//! proptest の any::<u32>() 256 回では、見つからなかった入力を、fuzzer は見つけられるか。
#![no_main]

use domain_lab::domain::NoteQuery;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|page: u32| {
    let query = NoteQuery {
        keyword: None,
        page,
    };
    let _ = query.offset();
});
