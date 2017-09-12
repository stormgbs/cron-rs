extern crate time;
extern crate cron_rs;

use std::thread;
use std::time as stdtime;
use std::process::Command;

use cron_rs::Scheduler;
use cron_rs::Job;
use cron_rs::Server;
use cron_rs::OutputKeeper;
use cron_rs::CronEntry;
use cron_rs::Task;

fn main() {
    // Make a time scheduler
    let tm = time::now();
    let sch = Scheduler::new("*/2 1-4,16,11,17 * * *").unwrap();
    println!("{:?}", sch);
    println!("{:?} {}", &tm, sch.isTimeUp(&tm));

    // Parse cron string to CronEntry
    let cronEntry = r#"{"intervals":"* * * * *","command":"date +%F_%T","description":"print time every minute","daemon":false,"testRun":true,"timeoutSeconds":5,"autokill":true,"alarmEmail":false,"alarmSms":true,"alarmUsers":"gaobushuang","id":1,"cronId":1,"treeId":261,"treePath":"b2c.b2cop.build-ci.build-ci.cn-test","active":false,"updateTime":"2017-02-07T13:22:52+08:00","lastCode":0,"lastJob":0,"watch":true,"host":""}"#
        .parse::<CronEntry>().unwrap();

    // Make time scheduler
    let sch = Scheduler::new(cronEntry.intervals).unwrap();

    // Make a new server and a output keeper,
    // then waiting for incoming message.
    let srv = Server {
        output_keeper: "foo".to_string(),
    };
    let mut keeper = OutputKeeper::new(&srv);

    // Make a new task
    let mut mytask = Task::new(sch, cronEntry, &keeper);
    // mytask spawn a new job periodically.
    mytask.start();

    // // This is a raw job.
    // let mut cmd = Command::new("ls");
    // cmd.arg("-al")
    //     .arg(".");
    //
    // let mut job = Job::new(100, cmd, &keeper);
    // job.start();
    //
    // // Take a while for receiving output
    // thread::sleep(stdtime::Duration::from_millis(20000));
}
