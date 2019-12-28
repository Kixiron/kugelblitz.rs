use log::{error, info};
use serenity::client::Client;

mod commands;
mod data;
mod event_handler;
mod logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    logger::setup_logger()?;

    let mut client = {
        use std::env;

        let token = match env::var("DISCORD_TOKEN") {
            Ok(token) => token,
            Err(err) => {
                error!("No discord token was supplied in the .env file!");
                error!("Make sure the .env file exists next to the executable and contains the following line:");
                error!("DISCORD_TOKEN=YOUR_DISCORD_TOKEN");
                return Err(Box::new(err));
            }
        };

        info!("Validating discord token...");
        serenity::client::validate_token(&token)?;

        info!("Creating discord client...");
        Client::new(&token, event_handler::Handler)?
    };

    info!("Setting up internal data...");
    data::setup_data(&mut client)?;

    let (owners, bot_id, bot_name) = {
        let info = client.cache_and_http.http.get_current_application_info()?;

        let mut owners = std::collections::HashSet::new();
        owners.insert(info.owner.id);

        (owners, info.id, info.name)
    };

    client.with_framework(commands::setup_framework(owners, bot_id));

    info!("Starting {}...", bot_name);
    client.start()?;

    Ok(())
}
