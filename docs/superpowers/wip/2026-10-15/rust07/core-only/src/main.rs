use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    // features = ["core"] だけの libsql で、リモートの DB を開こうとする
    let db = Builder::new_remote(
        "libsql://rustnote-example.turso.io".to_string(),
        String::new(),
    )
    .build()
    .await?;
    let _conn = db.connect()?;
    Ok(())
}
