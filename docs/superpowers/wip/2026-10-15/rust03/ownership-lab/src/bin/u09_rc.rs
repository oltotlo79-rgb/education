use std::rc::Rc;

fn main() {
    // Rc::new で、ヒープに「本体 + 持ち主の人数」を置く
    let shared = Rc::new(String::from("共有メモ"));
    println!("作成直後: 持ち主 {} 人", Rc::strong_count(&shared));

    // Rc::clone は本体を複製しない。人数を1増やして、同じ場所を指す持ち手を作るだけ
    let a = Rc::clone(&shared);
    println!("a を作成: 持ち主 {} 人", Rc::strong_count(&shared));
    {
        let b = Rc::clone(&shared);
        println!("b を作成: 持ち主 {} 人 / b = {b}", Rc::strong_count(&shared));
        println!("shared と b は同じ本体? {}", Rc::ptr_eq(&shared, &b));
    }
    println!("b が消えた: 持ち主 {} 人", Rc::strong_count(&shared));
    drop(a);
    println!("a を drop: 持ち主 {} 人", Rc::strong_count(&shared));
    // main の終わりで shared も消え、人数が 0 になった時点で本体が片付けられる
}
