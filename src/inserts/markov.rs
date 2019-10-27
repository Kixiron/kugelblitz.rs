use log::error;
use markov::Chain;
use serenity::prelude::TypeMapKey;
use std::{
    env,
    sync::{Arc, RwLock},
};

lazy_static::lazy_static! {
    static ref MARKOV_PATH: String = env::var("DISCORD_BOT_MARKOV").expect("");
}

pub struct Markov {
    chain: Chain<String>,
}

impl Markov {
    pub fn new() -> Self {
        let chain = match Chain::load(&*MARKOV_PATH) {
            Ok(chain) => chain,
            Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => Chain::new(),
            Err(err) => {
                error!("Error loading Markov Chain: {:?}", err);
                panic!("Error loading Markov Chain: {:?}", err);
            }
        };

        Self { chain }
    }

    #[inline]
    pub fn generate_str(&self) -> String {
        self.chain.generate_str()
    }

    #[inline]
    pub fn generate_str_from_token(&self, string: &str) -> String {
        self.chain.generate_str_from_token(string)
    }

    #[inline]
    pub fn feed_str(&mut self, string: &str) -> &mut Self {
        self.chain.feed_str(string);
        self
    }

    #[inline]
    pub fn save(&self) -> Result<(), std::io::Error> {
        self.chain.save(&*MARKOV_PATH)
    }
}

// TODO: Add timed saves

impl Drop for Markov {
    fn drop(&mut self) {
        if let Err(err) = self.save() {
            error!("Error Saving Markov: {:?}", err);
        }
    }
}

pub struct MarkovKey;

impl TypeMapKey for MarkovKey {
    type Value = Arc<RwLock<Markov>>;
}
