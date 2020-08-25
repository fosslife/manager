use tbot::Bot;
mod actions;

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("MANAGER_TOKEN").event_loop();

    bot.command("ban", actions::ban::ban);

    bot.command("mute", actions::mute::mute);

    bot.polling().start().await.unwrap();
}
