#[derive(Clone, Copy)]
struct NewNote {
    title: String,
}

fn main() {
    let a = NewNote {
        title: String::from("買い物"),
    };
    let b = a;
    println!("{} {}", a.title, b.title);
}
