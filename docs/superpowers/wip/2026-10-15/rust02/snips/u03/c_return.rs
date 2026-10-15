// 平均点を返す。人数が 0 のときは割り算できないので、先に 0 を返して終わる
fn average(total: u32, count: u32) -> u32 {
    if count == 0 {
        // return は「ここで関数を終えて、この値を返す」
        return 0;
    }
    total / count
}

fn main() {
    println!("{}", average(270, 3));
    println!("{}", average(0, 0));
}
