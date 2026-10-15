fn make_title(id: u32) -> String {
    format!("ノート{id}")
}

fn main() {
    println!("{}", make_title(1));
}
