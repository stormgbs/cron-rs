use scheduler::Scheduler;
use cron::CronEntry;

use task::Task;

pub struct Job {
    scheduler: Scheduler,
    cron: CronEntry,
}

impl Job {
    pub fn new(sch: Scheduler, cron: CronEntry) -> Job {
        Job {
            scheduler: sch,
            cron: cron,
        }
    }

    pub fn do_job(&self) -> Option<Task> {
        None
    }
}
