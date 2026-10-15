// データベースの「トランザクション」を真似した型
struct Transaction {
    id: u32,
    committed: bool,
}

impl Transaction {
    fn begin(id: u32) -> Transaction {
        println!("[{id}] BEGIN");
        Transaction {
            id,
            committed: false,
        }
    }

    // self を丸ごと受け取るので、commit した後の tx はもう使えない
    fn commit(mut self) {
        println!("[{}] COMMIT", self.id);
        self.committed = true;
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        // commit されないまま片付けられたら、取り消す
        if !self.committed {
            println!("[{}] ROLLBACK", self.id);
        }
    }
}

fn save_note(id: u32, title: &str) -> Result<(), String> {
    let tx = Transaction::begin(id);
    if title.is_empty() {
        // ここで return しても、tx の片付け (drop) は自動で走る
        return Err(String::from("タイトルが空です"));
    }
    println!("[{id}] INSERT INTO notes ... ({title})");
    tx.commit();
    Ok(())
}

fn main() {
    println!("{:?}", save_note(1, "買い物メモ"));
    println!("{:?}", save_note(2, ""));
}
