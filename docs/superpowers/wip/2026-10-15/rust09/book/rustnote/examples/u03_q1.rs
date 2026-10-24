//! 問1の解答: ノートに「優先度」（1 = 高・2 = 普通・3 = 低）を足すときの、型と DB の守り。
//! 実行: cargo run -q --example u03_q1

use rustnote::db::{self, Db};

/// 優先度。DB には 1〜3 の数で入れる
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Priority {
    High,
    Normal,
    Low,
}

impl Priority {
    /// DB の数から作る。1〜3 以外なら Err
    fn from_db(value: i64) -> Result<Priority, String> {
        match value {
            1 => Ok(Priority::High),
            2 => Ok(Priority::Normal),
            3 => Ok(Priority::Low),
            other => Err(format!("優先度は 1〜3 です（{other} が来ました）")),
        }
    }

    /// DB に入れる数
    fn to_db(self) -> i64 {
        match self {
            Priority::High => 1,
            Priority::Normal => 2,
            Priority::Low => 3,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 型の守り: 1〜3 以外は Priority にならない
    for value in [1, 2, 4] {
        println!(
            "Priority::from_db({value}) = {:?}",
            Priority::from_db(value)
        );
    }
    println!("Priority::Low.to_db() = {}", Priority::Low.to_db());

    // DB の守り: マイグレーションに書く SQL で、列と CHECK を足す（ここでは直接実行する）
    let dir = tempfile::tempdir()?;
    let db = Db::open_local(dir.path().join("u03.db")).await?;
    db::migrate(&db).await?;
    let conn = db.connect().await?;
    conn.execute_batch(
        "ALTER TABLE notes ADD COLUMN priority INTEGER NOT NULL DEFAULT 2
             CHECK (priority BETWEEN 1 AND 3);
         INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'x');",
    )
    .await?;
    for value in [0, 2, 4] {
        let result = conn
            .execute(
                "INSERT INTO notes (user_id, title, priority) VALUES (1, '宿題', ?1)",
                [value],
            )
            .await;
        match result {
            Ok(_) => println!("priority = {value}: OK"),
            Err(error) => println!("priority = {value}: NG {error}"),
        }
    }
    Ok(())
}
