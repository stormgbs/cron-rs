use std::str::FromStr;
use std::process::Command;
use serde_json;

use error::Error;
use error::Error::ErrCronFormat;
use scheduler::{Scheduler, SchedulerResult};

#[derive(Serialize, Deserialize, Debug)]
pub struct CronEntry {
    pub id: u32,
    pub cronId: u32,
    pub treeId: u64,
    pub treePath: String,

    pub intervals: String,
    pub command: String,
    pub description: String,
    pub daemon: bool,
    pub testRun: bool,
    pub timeoutSeconds: u32,
    pub autokill: bool,
    pub alarmEmail: bool,
    pub alarmSms: bool,
    pub alarmUsers: String,
    pub host: String,

    pub watch: bool,
    pub active: bool,
}

impl CronEntry {
    pub fn to_command(&self) -> Command {
        let fields: Vec<&str> = self.command.split_whitespace().collect();
        let mut cmd = Command::new(fields[0]);
        cmd.args(&fields[1..]);
        cmd
    }

    pub fn to_scheduler(&self) -> SchedulerResult {
        Scheduler::new(&self.intervals)
    }
}

impl FromStr for CronEntry {
    type Err = Error;

    fn from_str(s: &str) -> Result<CronEntry, Error> {
        serde_json::from_str::<CronEntry>(s).map_err(|e| ErrCronFormat(e.to_string()))
    }
}

#[test]
fn test_from_str() {
    let e = r#"{"intervals":"* * * * *","command":"date +%F_%T","description":"每分钟打印详细时间","daemon":false,"testRun":true,"timeoutSeconds":5,"autokill":true,"alarmEmail":false,"alarmSms":true,"alarmUsers":"gaobushuang","id":1,"cronId":1,"treeId":261,"treePath":"b2c.b2cop.build-ci.build-ci.cn-test","active":false,"updateTime":"2017-02-07T13:22:52+08:00","lastCode":0,"lastJob":0,"watch":true,"host":""}"#
        .parse::<CronEntry>().unwrap();
    println!("{:?}", e);
}
