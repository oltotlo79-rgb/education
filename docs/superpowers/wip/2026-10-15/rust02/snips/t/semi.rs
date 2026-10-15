fn main() {
    let nothing = {
        let x = 1;
        x + 1;
    };
    println!("nothing の中身: {nothing:?}");
}
