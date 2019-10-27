#![deny(unsafe_code)]
#![warn(
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::complexity,
    clippy::style,
    clippy::correctness,
    rust_2018_idioms
)]
#![allow(clippy::multiple_crate_versions, clippy::module_name_repetitions)]

// Why do you make me do this diesel
// TODO: Fuck you, we're doing this my way
#[macro_use]
extern crate diesel;

#[macro_use]
mod macros;

mod commands;
mod event_handler;
mod inserts;

use log::{error, info, warn};

fn main() {
    use event_handler::Handler;
    use serenity::{framework::standard::StandardFramework, Client};
    use std::{collections::HashSet, env, panic};

    color_backtrace::install();

    // Load the .env file
    dotenv::dotenv()
        .expect("No .env file was provided! Try creating one with the proper variables inside.");

    // Set up the logger
    setup_logger().expect("Failed to setup the logger");

    // Set the panic hook to use the error macro
    panic::set_hook(Box::new(|info| {
        error!("Panic: {}", info);
    }));

    // Create the client from the DISCORD_BOT_TOKEN and Handler
    let discord_token = env::var("DISCORD_BOT_TOKEN").expect("No token was found in the environmental variables, try setting `DISCORD_BOT_TOKEN` to the bots' token.");
    let mut client = Client::new(&discord_token, Handler)
        .expect("Error creating the bot client, try checking to see if the token is valid");

    {
        use hey_listen::sync::ParallelDispatcher as Dispatcher;
        use inserts::{
            Database, DatabaseKey, DispatchEvent, DispatcherKey, HttpClient, HttpClientKey, Markov,
            MarkovKey, SchedulerKey, ShardManagerContainer, Uptime, UptimeKey,
        };
        use std::sync::{Arc, Mutex, RwLock};
        use white_rabbit::Scheduler;

        // Get the client data to add things to it
        let mut data = client.data.write();

        // Start the uptime counter
        let uptime = Arc::new(Uptime::new());
        data.insert::<UptimeKey>(uptime);
        info!("Inserted Uptime");

        // Scheduler
        let scheduler = Arc::new(RwLock::new(Scheduler::new(8)));
        data.insert::<SchedulerKey>(scheduler);
        info!("Inserted Scheduler");

        // Dispatcher
        let mut dispatcher: Dispatcher<DispatchEvent> = Dispatcher::default();
        dispatcher
            .num_threads(8)
            .expect("Could not construct threadpool for the Dispatcher");
        data.insert::<DispatcherKey>(Arc::new(RwLock::new(dispatcher)));
        info!("Inserted Dispatcher");

        // Shard Manager
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        info!("Inserted Shard Manager");

        // Http Client
        data.insert::<HttpClientKey>(Arc::new(Mutex::new(HttpClient::new())));
        info!("Inserted the Http Client");

        // Database
        data.insert::<DatabaseKey>(Arc::new(Mutex::new(Database::new())));
        info!("Inserted the Database");

        // Markov
        data.insert::<MarkovKey>(Arc::new(RwLock::new(Markov::new())));
        info!("Inserted the Markov Chain");

        // TODO: Command usage?
    }

    // Get the bots' id
    let (bot_id, owners, prefixes) = {
        use serenity::model::id::UserId;

        // Get the bot id from the client cache
        let bot_id = match client.cache_and_http.http.get_current_user() {
            Ok(info) => info.id,
            Err(err) => panic!("Could not access application info: {:?}", err),
        };

        info!("Bot id: {}", bot_id);

        // Get the owners from the env variable and turn into a HashSet of UserIds
        let owners: HashSet<UserId> = env::var("DISCORD_BOT_OWNERS")
            .expect("No bot owners were found in the environmental variables, try setting `DISCORD_BOT_OWNERS` to the bots' owners' ids, separating them by only a comma. If there is only one owner no commas are needed.")
            .split(',')
            .map(|owner| UserId(owner.parse::<u64>().unwrap_or_else(|_| panic!("Invalid Owner id: {}", owner))))
            .collect();

        info!(
            "Owner id(s): {}",
            owners
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        );

        let prefixes: Vec<String> = env::var("DISCORD_BOT_PREFIXES")
            .expect("No prefixes were found in the environmental variables, try setting `DISCORD_BOT_PREFIXES` to the desired bot prefixes, separating by only a comma. If there is only one prefix no commas are needed")
            .split(',')
            .map(ToOwned::to_owned)
            .collect();

        info!("Prefix(es): {}", prefixes.join(", "));

        (bot_id, owners, prefixes)
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|config| {
                config
                    // Prefixes
                    .prefixes(prefixes)
                    // Allow the bot to work in dms
                    .allow_dm(true)
                    // Ignore whitespace in commands
                    .with_whitespace(true)
                    // Make commands ignore case
                    .case_insensitivity(true)
                    // Ignore bots
                    .ignore_bots(true)
                    // Ignore webhooks
                    .ignore_webhooks(true)
                    // Give the bot owners privilege
                    .owners(owners)
                    // Allow the bot to react to itself being mentioned as well as prefixed commands
                    .on_mention(Some(bot_id))
            })
            .on_dispatch_error(|ctx, msg, error| {
                event_handler::configure_dispatch_error(ctx, msg, error)
            })
            .unrecognised_command(|ctx, msg, unrecognised_command_name| {
                if let Err(err) = msg.reply(
                    &ctx,
                    &format!(
                        "`{}` is not a recognised command!",
                        unrecognised_command_name
                    ),
                ) {
                    error!("Reply error: {:?}", err);
                }
            })
            .normal_message(|ctx, msg| {
                use inserts::MarkovKey;

                let mut ctx = ctx.data.write();

                // Feed the markov
                if let Some(markov) = ctx.get_mut::<MarkovKey>() {
                    if let Ok(mut markov) = markov.try_write() {
                        markov.feed_str(&msg.content);
                    }
                }
            })
            .after(|ctx, msg, string, result| {
                event_handler::after_command(ctx, msg, string, result)
            })
            .bucket("ping", |b| b.delay(5).time_span(30).limit(5))
            .bucket("uptime", |b| b.delay(5).time_span(30).limit(5))
            .bucket("playground", |b| b.delay(10).time_span(20).limit(3))
            .help(&commands::KUGELBLITZ_HELP)
            .group(&commands::GENERAL_GROUP)
            .group(&commands::BOT_GROUP)
            .group(&commands::ADMIN_GROUP)
            .group(&commands::OWNER_GROUP),
    );

    // Start with the Discord-recommended amount of shards
    if let Err(err) = client.start_autosharded() {
        error!("Client Error: {:?}", err);
        panic!("Client Error: {:?}", err);
    }
}

/// Setup the environmental logger
#[inline]
fn setup_logger() -> Result<(), fern::InitError> {
    use log::LevelFilter;
    use std::env;

    // Get the log file
    let log_destination = env::var("DISCORD_BOT_LOGGING_DESTINATION")
        .expect("No log destination was found in the environmental variables, try setting `DISCORD_BOT_LOGGING_DESTINATION` to the desired output file.");

    // Get the log level
    let mut default_level_used = false;
    let log_level = env::var("DISCORD_BOT_LOGGING_LEVEL").unwrap_or_else(|_| {
        default_level_used = true;
        "Info".to_owned()
    });

    // Turn the log level into an enum
    let log_level = match log_level.trim().to_lowercase().as_str() {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        invalid_level => panic!("{} is not a valid logging level!", invalid_level),
    };

    let mut dispatch = fern::Dispatch::new()
        // Format to message [DATE][TIME][CRATE::MODULE][LOGGING LEVEL] MESSAGE
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        // Set the log level to the requested log level
        .level(log_level)
        // Print to stdout
        .chain(std::io::stdout());

    // Write logs to log file
    if !log_destination.is_empty() {
        dispatch = dispatch.chain(fern::log_file(&log_destination)?);
    }

    dispatch.apply()?;

    {
        let info = if log_destination.is_empty() {
            ("", "")
        } else {
            (" and ", log_destination.as_str())
        };
        info!("Logging outputs to stdio{}{}", info.0, info.1,);
    }

    Ok(())
}
