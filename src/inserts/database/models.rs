use super::schema::command_usage;
use diesel::{AsChangeset, Insertable, Queryable};

#[derive(Queryable, AsChangeset, Insertable)]
#[table_name = "command_usage"]
pub struct CommandEntry {
    pub cmd_name: String,
    pub cmd_usages: i32,
}
