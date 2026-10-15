mod domain {
    use std::fmt;
    use std::str::FromStr;

    // ---- ID 型 ----
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NoteId(i64);

    #[derive(Debug, PartialEq, Eq)]
    pub enum IdError {
        NotANumber,
        NotPositive,
    }

    impl NoteId {
        // DB から読んだ値など、すでに正しいと分かっている数から作る
        pub fn new(value: i64) -> Result<NoteId, IdError> {
            if value <= 0 {
                return Err(IdError::NotPositive);
            }
            Ok(NoteId(value))
        }

        pub fn get(self) -> i64 {
            self.0
        }
    }

    // "42".parse::<NoteId>() と書けるようにする
    impl FromStr for NoteId {
        type Err = IdError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let n: i64 = s.parse().map_err(|_| IdError::NotANumber)?;
            NoteId::new(n)
        }
    }

    // ---- メールアドレス ----
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Email(String);

    #[derive(Debug, PartialEq, Eq)]
    pub enum EmailError {
        MissingAt,
        Malformed,
        TooLong,
    }

    impl Email {
        pub const MAX_LEN: usize = 254;

        pub fn parse(input: &str) -> Result<Email, EmailError> {
            // 前後の空白を除き、小文字にそろえる（同じ人を同じ値にするため）
            let normalized = input.trim().to_lowercase();
            if normalized.len() > Self::MAX_LEN {
                return Err(EmailError::TooLong);
            }
            let Some((local, domain)) = normalized.split_once('@') else {
                return Err(EmailError::MissingAt);
            };
            let bad = local.is_empty()
                || domain.is_empty()
                || domain.contains('@')
                || !domain.contains('.')
                || normalized.contains(char::is_whitespace);
            if bad {
                return Err(EmailError::Malformed);
            }
            Ok(Email(normalized))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl fmt::Display for Email {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.0)
        }
    }
}

use domain::{Email, NoteId};

fn main() {
    // URL の /notes/{id} の {id} 部分を想定
    for segment in ["42", "abc", "0", "-5"] {
        let result: Result<NoteId, _> = segment.parse();
        println!("{segment:>4} -> {result:?}");
    }

    for input in [
        "  Alice@Example.COM ",
        "alice.example.com",
        "a@b@c.com",
        "bob@localhost",
    ] {
        match Email::parse(input) {
            Ok(email) => println!("OK: {} ({}バイト)", email, email.as_str().len()),
            Err(e) => println!("NG: {input:?} -> {e:?}"),
        }
    }

    let id = NoteId::new(7).expect("7 は正の数");
    println!("DB に渡す値: {}", id.get());
}
