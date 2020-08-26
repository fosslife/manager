extern crate config;

use tbot::Bot;

mod actions;
mod settings;

#[tokio::main]
async fn main() {
    let settings = settings::load();
    // let settings = Arc::new(settings);

    let mut bot = Bot::new(settings.get_str("bot.token").unwrap()).stateful_event_loop(settings);

    bot.command("ban", actions::ban::ban);

    bot.command("mute", actions::mute::mute);

    //TODO: Add help
    // bot.help(handler)

    bot.new_members(|ctx, state| async move {
        let chats = state
            .get::<Vec<tbot::types::chat::Id>>("chat.chat_list")
            .unwrap();
        let currnt_id = ctx.chat.id;
        match chats.clone().into_iter().find(|e| e == &currnt_id) {
            Some(chat) => println!("Valid addition {:?}", chat),
            None => {
                ctx.bot
                    .send_message(ctx.chat.id, "This group is not part of network bye")
                    .call()
                    .await
                    .unwrap();
                ctx.bot.leave_chat(ctx.chat.id).call().await.unwrap();
            }
        }
    });

    bot.polling().start().await.unwrap();
}
