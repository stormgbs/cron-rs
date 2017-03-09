use std::str::FromStr;

use scheduler::Scheduler;
use cron::CronEntry;
use error::Error;

use task::Task;

pub struct Job {
    scheduler: Scheduler,
    cron: CronEntry,
    task: Option<Task>,
}

impl Job {
    pub fn new(sch: Scheduler, cron: CronEntry) -> Job {
        Job {
            scheduler: sch,
            cron: cron,
            task: None,
        }
    }

    pub fn do_job(&mut self) -> Option<Task> {
        None
    }

    pub fn is_running(&self) -> bool {
        self.task.is_some()
    }
}
