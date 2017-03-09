extern crate libc;
extern crate time;
extern crate regex;
extern crate serde_json;
extern crate hyper;

#[macro_use]
extern crate serde_derive;

mod error;
mod scheduler;
mod job;
mod cron;
mod task;

use scheduler::Scheduler;

fn main() {
    let cron01 = Scheduler::new("*/2 1-4,16,11,17 * * *").unwrap();
    let tm = time::now();

    println!("{:?}", cron01);
    println!("{:?} {}", &tm, cron01.isTimeUp(&tm));
}
