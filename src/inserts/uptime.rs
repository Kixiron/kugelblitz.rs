use chrono::{DateTime, Duration, Local};
use serenity::prelude::TypeMapKey;
use std::sync::Arc;

pub struct Uptime {
    started_at: DateTime<Local>,
}

impl Uptime {
    pub fn new() -> Self {
        Self {
            started_at: Local::now(),
        }
    }

    pub fn get(&self) -> Duration {
        Local::now().signed_duration_since(self.started_at)
    }
}

pub struct UptimeKey;

impl TypeMapKey for UptimeKey {
    type Value = Arc<Uptime>;
}
