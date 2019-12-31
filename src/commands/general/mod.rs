use serenity::framework::standard::macros::group;

mod avatar;
mod counter;
mod ping;
mod reminder;
mod serverinfo;

use avatar::*;
use counter::*;
use ping::*;
use reminder::*;
use serverinfo::*;

#[group]
#[commands(ping, avatar, remind, command_counter, serverinfo)]
struct General;
