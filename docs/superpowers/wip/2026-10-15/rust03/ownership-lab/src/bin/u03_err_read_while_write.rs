fn main() {
    let mut score = 10;
    let writer = &mut score;
    println!("score = {score}");
    *writer += 1;
}
