use rusqlite::{Connection, Result}; // params

pub async fn load() -> Result<Connection> {
    std::fs::create_dir_all("data").unwrap();
    let db = Connection::open("data/governor.db")?;
    // db.execute(
    //     "CREATE TABLE IF NOT EXISTS Config (

    //     )",
    //     params![],
    // )?;
    Ok(db)
}
