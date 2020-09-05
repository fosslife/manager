use std::sync::Arc;
use tbot::contexts;
use tbot::prelude::*;
// use tbot::types::message::text::EntityKind;
use rusqlite::params;
use tbot::types::user::Id;
use tbot::util::entities::{self, Entity};
use tokio::sync::Mutex;

pub async fn promote(
    ctx: Arc<contexts::Command<contexts::Text>>,
    state: Arc<Mutex<crate::Storages>>,
) {
    let state = state.lock().await;
    // let db = &state.db;
    if ctx.from.clone().unwrap().id.to_string() != state.config.bot.president {
        ctx.send_message("Only President can issue this command")
            .call()
            .await
            .unwrap();
        return;
    };

    let user_to_promote = entities::entities(&ctx.text);
    for entity in user_to_promote.iter() {
        match entity {
            Entity::Semantic(user) => {
                // println!("Promoting {:?} == {:?}", user.value, user.kind);
                if user.kind.is_some() {
                    let mut mention = user.value[0].value.clone();
                    mention.retain(|c| c != '@');
                    println!("Promoting: {}", mention);
                    let id = state
                        .db
                        .query_row(
                            "
                    SELECT userid FROM Users where username = ?1",
                            params![mention],
                            |row| {
                                let id: i64 = row.get(0).unwrap();
                                Ok(id)
                            },
                        )
                        .unwrap();
                    // Doesn't work in private chats
                    let res = ctx
                        .promote_chat_member(Id::from(id))
                        .can_change_info(true)
                        .can_delete_messages(true)
                        .can_invite_users(true)
                        .can_pin_messages(true)
                        .can_promote_members(false)
                        .can_restrict_members(true)
                        .call()
                        .await;
                    if let Err(e) = res {
                        dbg!(e);
                        ctx.send_message("Error while promoting user. check logs")
                            .call()
                            .await
                            .unwrap();
                        return;
                    }

                    state
                        .db
                        .execute(
                            "
                        UPDATE Users
                        SET status = 'admin'
                        WHERE userid = ?1
                    ",
                            params![id],
                        )
                        .unwrap();
                    ctx.send_message(&format!("Promoted @{} to admin", mention))
                        .call()
                        .await
                        .unwrap();
                }
            }
            _ => panic!(),
        }
    }
    // ctx.promote_chat_member().call().await.unwrap();
}
