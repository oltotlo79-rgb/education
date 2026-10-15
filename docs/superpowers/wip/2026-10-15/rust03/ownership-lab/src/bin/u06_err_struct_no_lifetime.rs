struct Highlight {
    line_no: usize,
    text: &str,
}

fn main() {
    let h = Highlight {
        line_no: 1,
        text: "hello",
    };
    println!("{} {}", h.line_no, h.text);
}
