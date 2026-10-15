// タプルの中身を入れ替えて返す
fn swap(pair: (i32, char)) -> (char, i32) {
    let (number, letter) = pair;
    (letter, number)
}

fn main() {
    println!("{:?}", swap((7, 'R')));
}
