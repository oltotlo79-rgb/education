fn max_score(scores: &[i32]) -> Option<i32> {
    let mut best: Option<i32> = None;
    for &s in scores {
        best = match best {
            Some(b) if b >= s => Some(b),
            _ => Some(s),
        };
    }
    best
}

fn main() {
    let scores = vec![72, 95, 88];
    println!("{:?}", max_score(&scores));
    println!("{:?}", max_score(&scores[..1]));
    println!("{:?}", max_score(&[]));
}
