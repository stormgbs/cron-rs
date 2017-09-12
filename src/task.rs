use std::str::FromStr;
use std::thread;
use std::time as stdtime;

use time;

use scheduler::Scheduler;
use cron::CronEntry;
use error::Error;

use job::Job;
use output_keeper::OutputKeeper;

pub struct Task<'a, 'b: 'a> {
    scheduler: Scheduler<'a>,
    cron: CronEntry,
    keeper: &'a OutputKeeper<'b>,
    job: Option<Job<'a, 'b>>,
}

impl<'a, 'b: 'a> Task<'a, 'b> {
    pub fn new(sch: Scheduler<'a>, cron: CronEntry, keeper: &'b OutputKeeper) -> Task<'a, 'b> {
        Task {
            scheduler: sch,
            cron: cron,
            job: None,
            keeper: keeper,
        }
    }

    // start() periodically make a new job and then execute it in background.
    pub fn start(&mut self) {
        loop {
            let tm = time::now();

            if self.scheduler.isTimeUp(&tm) {
                let mut job = Job::new(100, self.cron.to_command(), self.keeper);
                job.start();
            }

            // sleep
            thread::sleep(stdtime::Duration::from_millis(1000 * (60-tm.tm_sec)));
        }
    }

    // fn is_running(&self) -> bool {
    //     //TODO: fix it, and make it public
    //     false
    // }
}
