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
use cron::CronEntry;
use job::Job;

fn main() {
    // test time scheduler
    let tm = time::now();
    let sch = Scheduler::new("*/2 1-4,16,11,17 * * *").unwrap();

    println!("{:?}", sch);
    println!("{:?} {}", &tm, sch.isTimeUp(&tm));

    // parse cron string
    let cron = r#"{"intervals":"* * * * *","command":"date +%F_%T","description":"每分钟打印详细时间","daemon":false,"testRun":true,"timeoutSeconds":5,"autokill":true,"alarmEmail":false,"alarmSms":true,"alarmUsers":"gaobushuang","id":1,"cronId":1,"treeId":261,"treePath":"b2c.b2cop.build-ci.build-ci.cn-test","active":false,"updateTime":"2017-02-07T13:22:52+08:00","lastCode":0,"lastTask":0,"watch":true,"host":""}"#
        .parse::<CronEntry>().unwrap();

    // make a new server and a output keeper,
    // then waiting for incoming message
    let srv = Server { output_keeper: "foo".to_string() };
    let mut keeper = OutputKeeper::new(&srv);

    // make a job
    let mut myjob = Job::new(sch, cron, &keeper);
    myjob.do_job();

    // this is another task
    let mut cmd = Command::new("ls");
    cmd.arg("-al")
        .arg(".");
    let mut task = Task::new(100, cmd, &keeper);
    task.start();

    // take a while for receiving output
    thread::sleep(stdtime::Duration::from_millis(2000));
}
