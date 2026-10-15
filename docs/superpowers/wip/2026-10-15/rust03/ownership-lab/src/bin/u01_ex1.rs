struct Noisy {
    name: String,
}

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.name);
    }
}

fn make(name: &str) -> Noisy {
    Noisy {
        name: name.to_string(),
    }
}

fn main() {
    let _x = make("x");
    {
        let _y = make("y");
        let z = make("z");
        drop(z);
        println!("block end");
    }
    let _w = make("w");
    println!("main end");
}
