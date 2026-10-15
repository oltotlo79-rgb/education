fn main() {
    let mut list = vec![String::from("a"), String::from("b")];

    let first = &list[0];
    println!("最初: {first}");
    // first はこの後もう使わないので、共有借用はここで終わっている

    list.push(String::from("c"));
    println!("{list:?}");

    let r1 = &list;
    let r2 = &list;
    println!("{} {}", r1.len(), r2.len());
    // r1 と r2 の出番は終わった。だから可変借用を作れる

    let w = &mut list;
    w.clear();
    println!("{list:?}");
}
