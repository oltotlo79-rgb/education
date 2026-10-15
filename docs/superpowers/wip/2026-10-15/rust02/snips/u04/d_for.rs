fn main() {
    // 0..3 は 0, 1, 2（終わりの 3 は含まない）
    for i in 0..3 {
        println!("i = {i}");
    }

    // 1..=3 は 1, 2, 3（= を付けると終わりも含む）
    for i in 1..=3 {
        // print! は改行しない表示
        print!("{i} ");
    }
    // 改行だけを出す
    println!();

    // .rev() で逆順にし、.step_by(3) で 3 つおきに取り出す
    for i in (0..10).rev().step_by(3) {
        print!("{i} ");
    }
    println!();

    // 配列の要素を先頭から1つずつ取り出す
    let fruits = ["りんご", "みかん", "ぶどう"];
    for fruit in fruits {
        println!("{fruit}");
    }
}
