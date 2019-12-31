use super::data_prelude::*;
use sled::Db;
use std::convert::TryInto;

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

    pub fn increment(old: Option<&[u8]>) -> Option<Vec<u8>> {
        let number = match old {
            Some(bytes) => {
                let array: [u8; 8] = bytes.try_into().unwrap();
                let number = u64::from_be_bytes(array);
                number + 1
            }
            None => 0,
        };

        Some(number.to_be_bytes().to_vec())
    }
}

impl std::ops::Deref for Database {
    type Target = Db;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}
