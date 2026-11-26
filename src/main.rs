// mod presentation;
mod application;
mod infrastructure;
mod domain;
mod settings;

use domain::{*};
use settings::{*};

use std::ptr;


#[tokio::main]
async fn main() -> EResult<()> {
    let settings = Settings::new(ptr::null());

    Ok(())
}
