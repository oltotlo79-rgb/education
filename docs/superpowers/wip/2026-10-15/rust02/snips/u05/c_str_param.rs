// &str を受け取る関数は、String からも文字列リテラルからも呼べる
fn shout(text: &str) -> String {
    format!("{}！", text.to_uppercase())
}

fn main() {
    let owned = String::from("rust");
    // &owned と書くと、String を &str として「貸す」ことになる
    println!("{}", shout(&owned));
    // 文字列リテラルは、もともと &str
    println!("{}", shout("hello"));
    // 貸しただけなので、owned はこのあとも使える
    println!("owned はまだ使える: {owned}");
}
