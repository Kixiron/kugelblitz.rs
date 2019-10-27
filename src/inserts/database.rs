mod models;
pub mod schema;

use diesel::{connection::Connection, sqlite::SqliteConnection};
use serenity::prelude::TypeMapKey;
use std::sync::{Arc, Mutex};

pub use models::*;

pub struct Database {
    connection: SqliteConnection,
}

impl Database {
    pub fn new() -> Self {
        use std::env;

        let db_url = env::var("DATABASE_URL")
            .expect("No database URL was found in the environmental variables, try setting `DATABASE_URL` to the URL or path of the Sqlite database.");
        let connection = SqliteConnection::establish(&db_url)
            .expect("Failed to connect to the database, make sure it exists, `DATABASE_URL` is correct and the database is a valid Sqlite3 Database.");

        Self { connection }
    }

    // TODO: Internalize all methods
    pub const fn get_inner(&self) -> &SqliteConnection {
        &self.connection
    }

    // pub fn create_test<'a>(
    //     &self,
    //     id: i32,
    //     stringable: &'a str,
    //     large_text: &'a str,
    //     true_or_false: bool,
    // ) -> Result<(), diesel::result::Error> {
    //     use schema::test;
    //     use crate::diesel::RunQueryDsl;
    //
    //     let new_post = NewTest { id, stringable, large_text, true_or_false };
    //
    //     diesel::insert_into(test::table)
    //         .values(&new_post)
    //         .execute(&self.connection)
    // }

    // use crate::inserts::{schema::test, DatabaseKey, Test};
    // use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
    //
    // if let Some(connection) = data.get::<DatabaseKey>() {
    //     if let Ok(connection) = connection.lock() {
    //         connection.create_test(1, "t", "t", false);
    //
    //         let results = test::table
    //             .filter(test::true_or_false.eq(false))
    //             .limit(5)
    //             .load::<Test>((*connection).get_inner())
    //             .expect("Error loading posts");
    //
    //         println!("Displaying {} posts", results.len());
    //         for post in results {
    //             println!("{}", post.id);
    //             println!("----------\n");
    //             println!("{}", post.stringable);
    //         }
    //     }
    // }

    // pub fn increment_command(&self, command_name: &str) -> Result<(), diesel::result::Error> {
    //     diesel::insert_or_ignore_into(command_usage)
    //         .values(CommandEntry {
    //             cmd_name: command_name,
    //             cmd_usages:  += 1,
    //         })
    // }
}

// TODO: Save on drop
// TODO: Add timed saves
// TODO: Internalize interactions

pub struct DatabaseKey;

impl TypeMapKey for DatabaseKey {
    type Value = Arc<Mutex<Database>>;
}
