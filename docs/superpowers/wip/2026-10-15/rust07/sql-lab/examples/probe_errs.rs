use libsql::Builder;
#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let c = db.connect()?;
    c.execute_batch(include_str!("../sql/schema.sql")).await?;
    c.execute_batch(include_str!("../sql/seed.sql")).await?;
    // too few params
    let r = c
        .execute(
            "UPDATE notes SET title = ?1 WHERE id = ?2",
            libsql::params!["x"],
        )
        .await;
    println!("few: {r:?}");
    // too many
    let r = c
        .execute(
            "UPDATE notes SET title = ?1 WHERE id = ?2",
            libsql::params!["x", 1, 2],
        )
        .await;
    println!("many: {r:?}");
    // query few
    let r = c
        .query("SELECT id FROM notes WHERE id = ?1 AND user_id = ?2", [1])
        .await;
    println!("query few: {:?}", r.err());
    // named missing
    let r = c
        .execute(
            "UPDATE notes SET title = :title WHERE id = :id",
            libsql::named_params! {":title": "x"},
        )
        .await;
    println!("named missing: {r:?}");
    // named wrong name
    let r = c
        .execute(
            "UPDATE notes SET title = :title WHERE id = :id",
            libsql::named_params! {":title": "x", ":idd": 1},
        )
        .await;
    println!("named wrong: {r:?}");
    // execute a select
    let r = c.execute("SELECT id FROM notes", ()).await;
    println!("execute select: {r:?}");
    // two statements in query
    let r = c.query("SELECT 1; SELECT 2", ()).await;
    println!("two stmts query: {:?}", r.is_ok());
    let r = c
        .execute(
            "UPDATE notes SET done = 1 WHERE id = 1; DELETE FROM notes",
            (),
        )
        .await;
    println!("two stmts execute: {r:?}");
    let mut rows = c.query("SELECT COUNT(*) FROM notes", ()).await?;
    println!("count {}", rows.next().await?.unwrap().get::<i64>(0)?);
    // NULL into String
    let mut rows = c.query("SELECT NULL", ()).await?;
    let row = rows.next().await?.unwrap();
    println!("null->String: {:?}", row.get::<String>(0));
    println!("null->Option: {:?}", row.get::<Option<String>>(0));
    // index out of range
    let mut rows = c
        .query("SELECT id, title FROM notes WHERE id = 1", ())
        .await?;
    let row = rows.next().await?.unwrap();
    println!("idx 5: {:?}", row.get::<i64>(5));
    println!(
        "bool from done: {:?}",
        c.query("SELECT 2", ())
            .await?
            .next()
            .await?
            .unwrap()
            .get::<bool>(0)
    );
    // u32 from negative
    println!(
        "u32 from -1: {:?}",
        c.query("SELECT -1", ())
            .await?
            .next()
            .await?
            .unwrap()
            .get::<u32>(0)
    );
    println!(
        "i32 from big: {:?}",
        c.query("SELECT 5000000000", ())
            .await?
            .next()
            .await?
            .unwrap()
            .get::<i32>(0)
    );
    // type mismatch: get i64 from text -> panic?
    let row = c
        .query("SELECT title FROM notes WHERE id = 1", ())
        .await?
        .next()
        .await?
        .unwrap();
    println!("title as i64: {:?}", row.get::<i64>(0));
    Ok(())
}
