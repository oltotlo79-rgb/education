use std::fmt;

struct Secret(String);

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret(********)")
    }
}

fn main() {
    let token = Secret("very-secret".to_string());
    println!("{token:?}");
    // {} で表示しようとする
    println!("トークン: {token}");
}
