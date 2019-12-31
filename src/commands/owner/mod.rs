use serenity::framework::standard::macros::group;

mod avatar;
mod disk_usage;
mod lists;
mod logs;
mod name;
mod nickname;
mod restart;
mod shut_down;
mod status;

use avatar::*;
use disk_usage::*;
use lists::*;
use logs::*;
use name::*;
use nickname::*;
use restart::*;
use shut_down::*;
use status::*;

#[group]
#[owners_only]
#[prefixes("sudo")]
#[description = "Owner-only commands"]
#[commands(
    usage,
    avatar,
    name,
    restart,
    shutdown,
    nickname,
    reset_status,
    online,
    idle,
    dnd,
    invisible,
    logs,
    blacklist,
    unblacklist
)]
pub struct Owner;

// TODO: Upload latest log file, DM only
