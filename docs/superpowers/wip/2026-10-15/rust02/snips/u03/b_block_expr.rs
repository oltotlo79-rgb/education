fn main() {
    // { } のブロックも「値になる式」。最後の行の値がブロック全体の値になる
    let area = {
        let width = 3;
        let height = 4;
        width * height
    };
    println!("面積: {area}");

    // if も式なので、結果をそのまま変数に入れられる
    let size = if area >= 10 { "大" } else { "小" };
    println!("大きさ: {size}");

    // 最後が「文」（セミコロン付き）のブロックは、() という「空の値」になる
    let nothing = {
        println!("ブロックの中で表示だけする");
    };
    println!("nothing の中身: {nothing:?}");
}
