fn main() {
    // alloc 機能だけでも、文字列への変換は使える
    let v = serde_json::json!({ "title": "牛乳を買う", "done": false });
    println!("{}", serde_json::to_string(&v).unwrap());
}
