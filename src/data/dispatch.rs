use super::data_prelude::*;
use hey_listen::sync::ParallelDispatcher as Dispatcher;
use serenity::model::id::{MessageId, UserId};

pub use hey_listen::sync::ParallelDispatcherRequest as DispatcherRequest;

pub struct DispatchContainer;

impl DispatchContainer {
    pub fn new_dispatcher() -> Arc<RwLock<Dispatcher<DispatchEvent>>> {
        let mut dispatcher: Dispatcher<DispatchEvent> = Dispatcher::default();
        if let Err(err) = dispatcher.num_threads(4) {
            log::error!("Could not construct dispatcher threadpool: {:?}", err);
            std::process::exit(1);
        }

        Arc::new(RwLock::new(dispatcher))
    }
}

impl TypeMapKey for DispatchContainer {
    type Value = Arc<RwLock<Dispatcher<DispatchEvent>>>;
}

#[derive(Clone)]
pub enum DispatchEvent {
    ReactEvent(MessageId, UserId),
}

impl PartialEq for DispatchEvent {
    fn eq(&self, other: &DispatchEvent) -> bool {
        match (self, other) {
            (
                DispatchEvent::ReactEvent(self_message_id, self_user_id),
                DispatchEvent::ReactEvent(other_message_id, other_user_id),
            ) => self_message_id == other_message_id && self_user_id == other_user_id,
        }
    }
}

impl Eq for DispatchEvent {}

impl std::hash::Hash for DispatchEvent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            DispatchEvent::ReactEvent(msg_id, user_id) => {
                msg_id.hash(state);
                user_id.hash(state);
            }
        }
    }
}
