#[derive(Debug)]
struct Pair<'a> {
    key: &'a str,
    value: &'a str,
}

fn split_pair(line: &str) -> Option<Pair<'_>> {
    let (key, value) = line.split_once(':')?;
    Some(Pair {
        key: key.trim(),
        value: value.trim(),
    })
}

fn main() {
    let line = String::from("title: 買い物");
    if let Some(pair) = split_pair(&line) {
        println!("{} => {}", pair.key, pair.value);
    }
    println!("{:?}", split_pair("no colon"));
}
