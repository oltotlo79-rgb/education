fn main() {
    // match の腕に if（ガード）を付けて、条件を足せる
    for temp in [-3, 0, 18, 35] {
        let feeling = match temp {
            t if t < 0 => "氷点下",
            0 => "ちょうど 0 度",
            t if t >= 30 => "真夏日",
            _ => "過ごしやすい",
        };
        println!("{temp}度: {feeling}");
    }

    // 入れ子のループで、外側のループごと抜ける（ラベル付き break）
    'outer: for row in 1..=3 {
        for col in 1..=3 {
            if row * col == 4 {
                println!("{row} × {col} = 4 を見つけたので終了");
                break 'outer;
            }
            println!("{row} × {col} = {}", row * col);
        }
    }
}
