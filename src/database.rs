use rusqlite::{params, Connection, Result};

pub async fn load() -> Result<Connection> {
    std::fs::create_dir_all("data").unwrap();
    let db = Connection::open("data/governor.db")?;
    //TODO: user leave handler
    //TODO: run indexes in prod only
    db.execute(
        "CREATE TABLE IF NOT EXISTS Users (
                userid          INTEGER PRIMARY KEY,
                username        TEXT,
                first_name      TEXT NOT NULL,
                last_name       TEXT,
                status          TEXT,
                joined_date     INTEGER,
                left_date       INTEGER

        )",
        params![],
    )?;
    // db.execute("
    //     CREATE INDEX idx_user_id
    //     ON Users (userid);
    // ", params![])?;
    Ok(db)
}
