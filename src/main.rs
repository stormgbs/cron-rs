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
mod job_master;
mod output_chunk;
mod output_keeper;
mod server;

use std::process::Command;

use scheduler::Scheduler;
use task::Task;
use std::thread;
use std::time as stdtime;
use server::Server;
use output_keeper::OutputKeeper;

fn main() {
    let cron01 = Scheduler::new("*/2 1-4,16,11,17 * * *").unwrap();
    let tm = time::now();

    println!("{:?}", cron01);
    println!("{:?} {}", &tm, cron01.isTimeUp(&tm));

    // make a new server
    let srv = Server { output_keeper: "foo".to_string() };

    // make a output keeper
    // watch for incoming message
    let mut keeper = OutputKeeper::new(&srv);

    // Task
    let mut cmd = Command::new("ls");
    cmd.arg("-al")
        .arg(".");
    let mut task = Task::new(100, cmd);
    task.start(keeper.tx.clone());

    thread::sleep(stdtime::Duration::from_millis(4000));
}
