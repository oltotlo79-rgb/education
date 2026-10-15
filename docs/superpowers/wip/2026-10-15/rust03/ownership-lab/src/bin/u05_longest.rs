// 返す参照は、x と y の「両方が有効な間」だけ有効、という約束
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("ownership");
    {
        let b = String::from("borrow");
        let result = longest(&a, &b);
        // a も b も生きている間に使うので OK
        println!("長いほう: {result}");
    }

    // 文字列リテラルはプログラムの最後まで有効なので、どちらと組んでも OK
    let result2 = longest(&a, "lifetime annotation");
    println!("長いほう: {result2}");
}
