fn make_title(id: u32) -> &'static str {
    let title = format!("ノート{id}");
    &title
}

fn main() {
    println!("{}", make_title(1));
}
