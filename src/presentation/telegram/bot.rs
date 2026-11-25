use teloxide::prelude::*;
use teloxide::types::User;

pub type Error = Box<dyn std::error::Error + Send + Sync>;



struct TelegramBot {
    bot: Bot,
    schema: Update,
    dispatcher: Dispatcher
}

impl TelegramBot {
    pub async fn new(token: String) -> Result<Self, Error> {
        let bot = Bot::new(token);

        Self {
            bot: bot
        }
    }

    pub async fn process_text_message(bot: Bot, user: User, message_text: String) -> Result<(), Error> {
        /*
        The id of a chat with a user is the same as his telegram_id
        from the bot's perspective.

        Injected dependencies:
        - Bot is provided by the Dispatcher::dispatch
        - User is provided by the (1)
        - String is provided by the (2)
        */
        bot.send_message(user.id, format!("Hi! You sent: {message_text}"));
        Ok(())
    }

    pub async fn run() -> Result<(), Error> {
        Dispatcher::builder(bot, schema).build().dispatch().await;
        Ok(())
    }
}
