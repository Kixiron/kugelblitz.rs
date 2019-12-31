use super::data_prelude::*;
use white_rabbit::Scheduler;

pub struct SchedulerContainer;

impl SchedulerContainer {
    pub fn new_scheduler() -> Arc<RwLock<Scheduler>> {
        let scheduler = Scheduler::new(4);

        Arc::new(RwLock::new(scheduler))
    }
}

impl TypeMapKey for SchedulerContainer {
    type Value = Arc<RwLock<Scheduler>>;
}
