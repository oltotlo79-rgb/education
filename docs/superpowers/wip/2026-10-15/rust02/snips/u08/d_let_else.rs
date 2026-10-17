// "key=value" の形の文字列から value を取り出す。形が違えば None
fn value_of(pair: &str) -> Option<&str> {
    // split_once は、最初の '=' で2つに分ける。'=' がなければ None
    let Some((_key, value)) = pair.split_once('=') else {
        return None;
    };
    Some(value.trim())
}

fn main() {
    println!("{:?}", value_of("page= 3"));
    println!("{:?}", value_of("page"));
}
