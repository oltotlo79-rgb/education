struct Noisy;

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("片付け");
    }
}

fn main() {
    let a = Noisy;
    a.drop();
}
