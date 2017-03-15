use std::str::FromStr;

use scheduler::Scheduler;
use cron::CronEntry;
use error::Error;

use task::Task;
use output_keeper::OutputKeeper;

pub struct Job<'a, 'b: 'a> {
    scheduler: Scheduler<'a>,
    cron: CronEntry,
    keeper: &'a OutputKeeper<'b>,
    task: Option<Task<'a, 'b>>,
}

impl<'a, 'b: 'a> Job<'a, 'b> {
    pub fn new(sch: Scheduler<'a>, cron: CronEntry, keeper: &'b OutputKeeper) -> Job<'a, 'b> {
        Job {
            scheduler: sch,
            cron: cron,
            task: None,
            keeper: keeper,
        }
    }

    pub fn do_job(&mut self) {
        let mut task = Task::new(100, self.cron.to_command(), self.keeper);
        task.start();
    }

    pub fn is_running(&self) -> bool {
        //TODO
        false
    }
}
