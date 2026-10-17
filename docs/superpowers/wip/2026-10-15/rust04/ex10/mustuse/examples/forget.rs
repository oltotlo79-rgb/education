use toggle::Progress;

fn main() {
    let progress = Progress::Open;
    // 結果を受け取らずに捨ててしまう（progress 自体は変わらない）
    progress.toggle();
    println!("{progress:?}");
}
