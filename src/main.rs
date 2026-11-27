#![feature(min_adt_const_params)]

use tokio;

// mod presentation;
mod application;
mod infrastructure;
mod presentation;
mod domain;
mod settings;

use domain::{*};
use presentation::telegram::{*};
use settings::Settings;

use std::ptr;


#[tokio::main]
async fn main() -> EResult<()> {
    let settings = Settings::new(None)?;
    
    let bot = TelegramBot::new(settings.tbot_token.value().to_string()).await?;
    bot.run().await?;
    Ok(())
}
