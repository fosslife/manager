use tbot::{prelude::*, Bot};

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("MANAGER_TOKEN").event_loop();

    bot.command("kick", |ctx| async move {
        if let Some(message) = ctx.reply_to.clone() {
            let user_to_kick = message.from.as_ref().unwrap().id;
            let username = message.from.as_ref().unwrap().username.as_ref().unwrap();
            let chat_id = ctx.chat.id;
            ctx.bot
                .kick_chat_member(chat_id, user_to_kick)
                .call()
                .await
                .unwrap();
            ctx.send_message(&format!("Kicked user {}", username)).call().await.unwrap();
            
        } else {
            // TODO: add /kick userid API
            ctx.send_message("Please reply to a message that you want to kick!").call().await.unwrap();
        }
        
    });

    bot.polling().start().await.unwrap();
}
