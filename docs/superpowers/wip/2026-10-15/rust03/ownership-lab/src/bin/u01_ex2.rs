struct LockGuard {
    resource: String,
}

impl LockGuard {
    fn acquire(resource: &str) -> LockGuard {
        println!("ロック取得: {resource}");
        LockGuard {
            resource: resource.to_string(),
        }
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        println!("ロック解除: {}", self.resource);
    }
}

fn update(value: i32) -> Result<i32, String> {
    let _guard = LockGuard::acquire("notes");
    if value < 0 {
        return Err(String::from("負の数は保存できません"));
    }
    println!("更新: {value}");
    Ok(value)
}

fn main() {
    println!("{:?}", update(5));
    println!("{:?}", update(-1));
}
