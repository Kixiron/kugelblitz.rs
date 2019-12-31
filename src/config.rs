use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Config {
    pub discord_token: String,
    pub blacklisted_users: Vec<u64>,
    pub blacklisted_servers: Vec<u64>,
}

impl Config {
    pub fn load() -> Result<Self, &'static str> {
        let contents = {
            let mut file = File::open("./Config.toml")
                .or_else(|_| Err("Could not find a config file, make sure Config.toml exists"))?;

            let mut contents = if let Ok(meta) = file.metadata() {
                String::with_capacity(meta.len() as usize)
            } else {
                String::new()
            };

            file.read_to_string(&mut contents)
                .or_else(|_| Err("Failed to read the contents of Config.toml"))?;

            contents
        };

        let mut config: Config = toml::from_str(&contents)
            .or_else(|_| Err("Failed to read Config.toml, make sure that the format is correct"))?;

        config.blacklisted_users.sort_unstable();
        config.blacklisted_servers.sort_unstable();

        Ok(config)
    }
}
