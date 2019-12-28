use super::data_prelude::*;
use sled::Db;

pub struct DatabaseContainer;
impl TypeMapKey for DatabaseContainer {
    type Value = Arc<RwLock<Database>>;
}

pub struct Database {
    db: Db,
}

impl Database {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db = sled::open("./data/database")?;

        Ok(Self { db })
    }

    pub fn to_bytes<T: serde::Serialize>(data: &T) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let serialized = bincode::serialize(data)?;
        Ok(serialized)
    }

    pub fn from_bytes<'de, T: serde::Deserialize<'de>>(
        bytes: &'de [u8],
    ) -> Result<T, Box<dyn std::error::Error>> {
        let deserialized = bincode::deserialize(bytes)?;
        Ok(deserialized)
    }
}

impl std::ops::Deref for Database {
    type Target = Db;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}
