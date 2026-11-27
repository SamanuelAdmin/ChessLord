use teloxide::{prelude::*, utils::command::BotCommands};


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported: ")]
pub enum DMCommand {
    #[command(description = "start the bot")]
    Start,
    #[command(description = "display this text.")]
    Help
}


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported admin's commands: ")]
pub enum AdminCommand {
    #[command(description = "start bot for a group")]
    Start,
    #[command(description = "sending frequency")]
    Frequency(u8),
    #[command(description = "puzzles rating")]
    Rating(u16),
    #[command(description = "stop bot for a group")]
    Stop
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported user's commands: ")]
pub enum UserCommand {
    #[command(description = "help message")]
    Help,
    #[command(description = "answer for the puzzle")]
    Answer(String)
}

