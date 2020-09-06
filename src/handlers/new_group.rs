use rusqlite::params;
use rusqlite::types::Null;
use std::sync::Arc;
use std::time::SystemTime;
use tbot::contexts;
use tbot::prelude::*;
use tokio::sync::Mutex;

pub async fn new_memeber(ctx: Arc<contexts::NewMembers>, state: Arc<Mutex<crate::Storages>>) {
    let state = state.lock().await;
    // let db = &state.lock().await.db;

    let user = ctx.from.as_ref().unwrap();
    info!("Got new Member {}", &user.first_name);

    let status = "member";

    let joined_date = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let find_user = state.db.query_row(
        "
        SELECT userid, status FROM Users
        WHERE userid = ?1",
        params![user.id.0],
        |row| {
            let id: i64 = row.get(0).unwrap();
            let status: String = row.get(1).unwrap();
            Ok((id, status))
        },
    );

    if let Ok(prev_user) = find_user {
        if prev_user.1 == "admin".to_string() {
            info!("User was admin previously, promoting in this group");
            let res = ctx
                .promote_chat_member(ctx.from.clone().unwrap().id)
                .can_change_info(true)
                .can_delete_messages(true)
                .can_invite_users(true)
                .can_pin_messages(true)
                .can_promote_members(false)
                .can_restrict_members(true)
                .call()
                .await;
            if let Err(e) = res {
                debug!("Error in promoting user {}", e);
                ctx.send_message("Error while promoting joined user. check logs")
                    .call()
                    .await
                    .unwrap();
                return;
            }
            return;
        } else {
            info!("User was not admin previously, updating record with new information");
            let res = state.db.execute(
                "
                INSERT OR REPLACE INTO Users
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
                    info!("Rows written: {}", rows);
                }
                Err(e) => {
                    error!("Error occurred while saving users {}", e);
                }
            }
        }
    }

    info!("Added user to database");
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
