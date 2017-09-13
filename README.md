# cron_rs  
![Rust](https://img.shields.io/badge/rust-nightly-red.svg)
[![Crates.io](https://img.shields.io/crates/d/cron_rs.svg)](https://crates.io/crates/cron_rs)
[![Docs.rs](https://docs.rs/cron_rs/badge.svg)](https://docs.rs/cron_rs)
> Cron parser and periodic jobs scheduler that are written in rust.

## USAGE

### Cron Time Format

`  
MINUTE HOUR DAY MONTH WEEKDAY  
`  

* MINUTE (required): minute, 0-59.
* HOUR (required): hour, 0-23.
* DAY (required): day of month, 1-31.
* MONTH (required): month of year, 1-12.
* WEEKDAY (required): day of week, 0-6.

### Installation  

In Cargo.toml:  

`  
    [dependencies]  
    cron_rs = "*"  
`  

#### Good Example: Cron Time Format (Intervals)
`  
*/2 1-8,11 * * *  
`  

### A Full Example  

Scheduler is a cron time shceduler.  
The example below will show you:  
* Parse CronEntry.  
* Make a output keeper to capture cron outputs.
* Make a scheduler.  
* Make a task, and then start it, it will spawn and excute new job periodically.  

`  
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
    let intervals = &*cronEntry.intervals.to_owned();
    let sch = Scheduler::new(intervals).unwrap();

    // Make a new server and a output keeper,
    // then waiting for incoming message.
    let srv = Server {
        output_keeper: "foo".to_string(),
    };
    let mut keeper = OutputKeeper::new(&srv);

    // Make a new task
    let mut mytask = Task::new(sch, cronEntry, &keeper);

    // mytask spawn a new job and execute it every minute.
    mytask.start();
}  
`  

## DEPENDENICIES

* time
* regex

## FAQ
