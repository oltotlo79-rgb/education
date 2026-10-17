use std::collections::HashMap;

fn main() {
    // キー（ID）から値（タイトル）を引く表を作る
    let mut titles: HashMap<u64, String> = HashMap::new();
    titles.insert(1, String::from("牛乳を買う"));
    titles.insert(2, String::from("Rust の勉強"));
    titles.insert(3, String::from("部屋の掃除"));

    // get は、キーがあれば Some(&値)、なければ None
    println!("1: {:?}", titles.get(&1));
    println!("9: {:?}", titles.get(&9));
    println!("2 はある? {}", titles.contains_key(&2));

    // 同じキーで insert すると上書きされ、前の値が Some で返る
    let old = titles.insert(2, String::from("Rust の復習"));
    println!("上書き前の値: {old:?}");

    // remove は、取り除いた値を返す
    let removed = titles.remove(&3);
    println!("削除: {removed:?}, 残り {} 件", titles.len());

    // HashMap の順番は決まっていないので、表示するときはキーで並べ替える
    let mut ids: Vec<&u64> = titles.keys().collect();
    ids.sort();
    for id in ids {
        println!("#{id} {}", titles[id]);
    }
}
