//! 第11巻 UNIT 04: RustNote の検査済みの型の parse に、fuzzer が作るでたらめな文字列を、次々に渡す。
//! パニックしないことと、「合格した値を、もう一度 parse しても同じ」を確かめる。
#![no_main]

use domain_lab::domain::{Email, NoteBody, NoteId, NoteQuery, NoteTitle, Password};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: &str| {
    if let Ok(title) = NoteTitle::parse(input) {
        assert_eq!(NoteTitle::parse(title.as_str()), Ok(title));
    }
    if let Ok(email) = Email::parse(input) {
        assert_eq!(Email::parse(email.as_str()), Ok(email));
    }
    let _ = NoteBody::parse(input);
    let _ = Password::parse(input);
    let _ = input.parse::<NoteId>();
    let query = NoteQuery::new(Some(input), None);
    let _ = query.offset();
});
