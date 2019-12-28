use super::data_prelude::*;
use chrono::Utc;
use white_rabbit::{DateResult, Duration, Scheduler};

pub struct SchedulerContainer;

impl SchedulerContainer {
    fn new_scheduler() -> Arc<Mutex<Scheduler>> {
        let mut scheduler = Scheduler::new(4);

        scheduler.add_task_duration(Duration::minutes(30), |_ctx| {
            log::info!("");

            DateResult::Repeat(Utc::now() + Duration::milliseconds(5000))
        });

        Arc::new(Mutex::new(scheduler))
    }
}

impl TypeMapKey for SchedulerContainer {
    type Value = Arc<Mutex<Scheduler>>;
}
