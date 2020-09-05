use rusqlite::params;
use rusqlite::types::Null;
use std::sync::Arc;
use std::time::SystemTime;
use tbot::contexts;
use tokio::sync::Mutex;
// use tbot::prelude::*;

pub async fn new_memeber(ctx: Arc<contexts::NewMembers>, state: Arc<Mutex<crate::Storages>>) {
    let db = &state.lock().await.db;

    let user = ctx.from.as_ref().unwrap();
    let status = "member";
    let joined_date = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    let res = db.execute(
        "
        INSERT OR IGNORE INTO Users
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
    ",
        params![
            user.id.0,
            user.username,
            user.first_name,
            user.last_name,
            status,
            joined_date as u32,
            Null
        ],
    );

    match res {
        Ok(rows) => {
            println!("Rows written: {}", rows);
        }
        Err(e) => {
            dbg!("Error occurred while saving users {}", e);
        }
    }
    // let currnt_id = ctx.chat.id;
    // match chats.clone().into_iter().find(|e| e == &currnt_id) {
    //     Some(chat) => println!("Valid addition {:?}", chat),
    //     None => {
    //         ctx.bot
    //             .send_message(ctx.chat.id, "This group is not part of network bye")
    //             .call()
    //             .await
    //             .unwrap();
    //         ctx.bot.leave_chat(ctx.chat.id).call().await.unwrap();
    //     }
    // }
}
