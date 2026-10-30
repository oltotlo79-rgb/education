//! 第11巻 UNIT 04 問3: NoteQuery を、URL から作るときと同じ NoteQuery::new で作り、offset を試す。
#![no_main]

use domain_lab::domain::NoteQuery;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|page: Option<u32>| {
    let query = NoteQuery::new(None, page);
    let _ = query.offset();
});
