use log::{error, info};
use serenity::client::Client;

mod commands;
mod config;
mod data;
mod event_handler;
mod logger;

lazy_static::lazy_static! {
    static ref OWNER_ID: std::sync::Arc<parking_lot::RwLock<u64>> =
        std::sync::Arc::new(parking_lot::RwLock::new(0));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::setup_logger()?;

    #[cfg(not(debug_assertions))]
    spawn_deadlock_detection()?;

    info!("Loading config...");
    let config = config::Config::load()?;

    let mut client = {
        info!("Validating discord token...");
        serenity::client::validate_token(&config.discord_token)?;

        info!("Creating discord client...");
        Client::new(&config.discord_token, event_handler::Handler)?
    };

    info!("Setting up internal data...");
    data::setup_data(&mut client, config)?;

    let (owners, bot_id, bot_name) = {
        let info = client.cache_and_http.http.get_current_application_info()?;

        let mut owners = std::collections::HashSet::new();
        owners.insert(info.owner.id);

        let mut owner_id = OWNER_ID.write();
        *owner_id = *info.owner.id.as_u64();

        (owners, info.id, info.name)
    };

    client.with_framework(commands::setup_framework(owners, bot_id));

    info!("Starting {}...", bot_name);
    client.start_autosharded()?;

    Ok(())
}

fn spawn_deadlock_detection() -> Result<(), Box<dyn std::error::Error>> {
    use parking_lot::deadlock;
    use std::{
        thread::{self, Builder},
        time::Duration,
    };

    Builder::new()
        .name("deadlock-detection".into())
        .spawn(move || loop {
            let deadlocks = deadlock::check_deadlock();
            if deadlocks.is_empty() {
                continue;
            }

            error!("{} deadlocks detected", deadlocks.len());
            for (i, threads) in deadlocks.iter().enumerate() {
                error!("Deadlock #{}", i);
                for t in threads {
                    error!("Thread Id {:#?}", t.thread_id());
                    error!("{:#?}", t.backtrace());
                }
            }

            thread::sleep(Duration::from_secs(10));
        })?;

    Ok(())
}
