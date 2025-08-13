use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use dotenv::dotenv;

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "start the bot")]
    Start,
    #[command(description = "show this help text")]
    Help,
}

async fn answer(bot: AutoSend<Bot>, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, "Hello World").await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions()).await?;
        }
    };
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    teloxide::enable_logging!();
    let bot = Bot::from_env().auto_send();
    teloxide::commands_repl(bot, "hello_bot", answer).await;
}
