use hey_listen::sync::ParallelDispatcher as Dispatcher;
use serenity::{model::id::MessageId, prelude::TypeMapKey};
use std::sync::{Arc, RwLock};
use white_rabbit::Scheduler;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum DispatchEvent {
    Message(MessageId),
}

pub struct DispatcherKey;

impl TypeMapKey for DispatcherKey {
    type Value = Arc<RwLock<Dispatcher<DispatchEvent>>>;
}

pub struct SchedulerKey;

impl TypeMapKey for SchedulerKey {
    type Value = Arc<RwLock<Scheduler>>;
}
