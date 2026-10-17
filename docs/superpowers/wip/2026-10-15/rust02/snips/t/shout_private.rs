mod text_utils {
    // pub を付けて、モジュールの外から呼べるようにする
    fn shout(text: &str) -> String {
        format!("{}！", text.to_uppercase())
    }
}

fn main() {
    println!("{}", text_utils::shout("rust"));
}
