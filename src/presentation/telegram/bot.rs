use teloxide::{
    dispatching::HandlerExt, prelude::*, sugar::request::RequestReplyExt, types::Dice,
    utils::command::BotCommands, dispatching::DefaultKey
};

use super::commands::{*};
use super::errors::{*};
use crate::domain::EResult;


pub struct TelegramBot {
    bot: Bot,
    dispatcher: Dispatcher<Bot, DispatcherError, DefaultKey>
}

impl TelegramBot {
    pub async fn new(token: String) -> EResult<Self> {
        let bot = Bot::new(token);

        let handler = dptree::entry()
            .branch(
                dptree::filter(|m: ChatMemberUpdated| {
                    m.old_chat_member.is_owner() || m.old_chat_member.is_administrator()
                })
                .filter_command::<AdminCommand>()
                .endpoint(|msg: Message, bot: Bot, cmd: AdminCommand| async move {
                    Self::admin_command_processor(msg, bot, cmd)
                }),
            );

        let dispatcher = Dispatcher::builder(bot, handler)
            .dependencies(dptree::deps![])
            .enable_ctrlc_handler()
            .build();

        Ok(Self {
            bot: bot,
            dispatcher: dispatcher
        })
    }

    pub async fn run(mut self) -> EResult<()> {
        self.dispatcher.dispatch().await; 
        Ok(())
    }

    pub async fn admin_command_processor(msg: Message, bot: Bot, cmd: AdminCommand) -> EResult<()> {
        match cmd {
            AdminCommand::Start => {
                bot.send_message(msg.chat.id, "Bot has been started by admin.".to_string()).await?;
                Ok(())
            }
            AdminCommand::Stop => {
                bot.send_message(msg.chat.id, "Bot has been stopped by admin.".to_string()).await?;
                Ok(())
            }
            AdminCommand::Frequency(new_freq) => {
                bot.send_message(msg.chat.id, format!("Changed frequency to {}", new_freq).to_string()).await?;
                Ok(())
            }
            AdminCommand::Rating(new_rating) => {
                bot.send_message(msg.chat.id, format!("Changed rating to {}", new_rating).to_string()).await?;
                Ok(())
            }
        }
    }
}
