struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // 構造体を分解して、フィールドの値に名前を付ける
    let p = Point { x: 3, y: -2 };
    let Point { x, y } = p;
    println!("x = {x}, y = {y}");

    // match で構造体の「形」を調べる
    let points = [
        Point { x: 0, y: 0 },
        Point { x: 0, y: 5 },
        Point { x: 2, y: -1 },
        Point { x: 4, y: 4 },
    ];
    for point in points {
        let place = match point {
            Point { x: 0, y: 0 } => "原点",
            // .. は「残りのフィールドは何でもよい」
            Point { x: 0, .. } => "y 軸の上",
            Point { y, .. } if y < 0 => "x 軸より下",
            _ => "その他",
        };
        println!("{place}");
    }

    // @ で「範囲に合った値」に名前を付ける
    let age = 17;
    match age {
        n @ 0..=17 => println!("{n} 歳は未成年"),
        n => println!("{n} 歳は成年"),
    }

    // matches! は「パターンに合うか」を true / false で返す
    let c = 'の';
    println!("ひらがな? {}", matches!(c, 'ぁ'..='ゖ'));
}
