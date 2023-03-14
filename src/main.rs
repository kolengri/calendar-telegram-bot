use teloxide::prelude::*;
use teloxide::enable_logging;
use teloxide::types::ParseMode::HTML;
use dotenv::dotenv;


async fn respond(cx: UpdateWithCx<Message>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Get the text of the incoming message
    let text = cx.update.text_owned().unwrap_or_else(|| "".to_string());

    // Send a greeting back to the user
    let response = format!("Hello, {}!", cx.update.from.first_name);
    cx.answer(response).parse_mode(HTML).send().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    enable_logging!();
    log::info!("Starting my_telegram_bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, respond).await;
}
