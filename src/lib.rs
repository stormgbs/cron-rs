extern crate time;
extern crate regex;

use std::io;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;
use regex::Regex;

type SchedulerResult = Result<Scheduler, String>;

#[derive(Debug)]
pub enum CronError {
    ParseFormat(&'static str),
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for CronError {
    fn from(err: ParseIntError) -> CronError {
        CronError::ParseInt(err)
    }
}

impl fmt::Display for CronError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &CronError::ParseFormat(x) => write!(f, "{:?}", x),
            &CronError::ParseInt(ref e) => write!(f, "{:?}", e),
        }
    }
}


#[derive(Debug)]
pub struct Scheduler {
    seconds: &'static str,
    minutes: &'static str,
    hours: &'static str,
    days: &'static str,
    months: &'static str,
    weekdays: &'static str,

    timeFiledsLength: usize,

    timePoints: HashMap<&'static str, HashSet<u32>>,

    re: Regex,
}

impl Scheduler {
    fn new(intervals: &'static str) -> SchedulerResult {
        let reRes = Regex::new(r"^\s*((\*(/\d+)?)|[0-9-,/]+)(\s+((\*(/\d+)?)|[0-9-,/]+)){4,5}\s*$");

        match reRes {
            Ok(re) => {
                if !re.is_match(intervals) {
                    return Err(format!("invalid format: {}", intervals));
                }

                let timeFileds: Vec<&'static str> = intervals.split_whitespace().collect();
                let timeFiledsLength = timeFileds.len();

                if timeFiledsLength != 5 && timeFiledsLength != 6 {
                    return Err(format!("length of itervals should be 5 or 6, but got {}",
                                       timeFiledsLength));
                }

                let mut sec = "";
                let mut startIndex: usize = 0;

                if timeFiledsLength == 6 {
                    sec = timeFileds[0].clone();
                    startIndex = 1;
                }

                let mut sch = Scheduler {
                    seconds: sec,
                    minutes: timeFileds[startIndex],
                    hours: timeFileds[startIndex + 1],
                    days: timeFileds[startIndex + 2],
                    months: timeFileds[startIndex + 3],
                    weekdays: timeFileds[startIndex + 4],
                    timeFiledsLength: timeFiledsLength,
                    timePoints: HashMap::new(),
                    re: re,
                };

                try!(sch.parseTimeFields().map_err(|e| e.to_string()));
                Ok(sch)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    fn parseTimeFields(&mut self) -> Result<(), CronError> {
        if self.seconds != "" {
            self.timePoints.insert("seconds", try!(parseTimeField(self.seconds, 0, 59)));
        } else {
            self.timePoints.insert("seconds", [0].iter().cloned().collect::<HashSet<u32>>());
        }

        self.timePoints.insert("minutes", try!(parseTimeField(self.minutes, 0, 59)));
        self.timePoints.insert("hours", try!(parseTimeField(self.hours, 0, 23)));
        self.timePoints.insert("days", try!(parseTimeField(self.days, 1, 31)));
        self.timePoints.insert("months", try!(parseTimeField(self.months, 1, 12)));
        self.timePoints.insert("weekdays", try!(parseTimeField(self.weekdays, 0, 6)));

        Ok(())
    }

    fn isTimeUp(&self, t: &time::Tm) -> bool {
        let (second, minute, hour, day, month, weekday) = (t.tm_sec as u32,
                                                           t.tm_min as u32,
                                                           t.tm_hour as u32,
                                                           t.tm_mday as u32,
                                                           t.tm_mon as u32,
                                                           t.tm_wday as u32);

        let isSecond = self.timePoints.get("seconds").unwrap().contains(&second);
        let isLeft = (self.timePoints.get("minutes").unwrap().contains(&minute) &&
                      self.timePoints.get("hours").unwrap().contains(&hour) &&
                      self.timePoints.get("days").unwrap().contains(&day) &&
                      self.timePoints.get("months").unwrap().contains(&month) &&
                      self.timePoints.get("weekdays").unwrap().contains(&weekday));

        if self.timeFiledsLength == 5 {
            true && isLeft
        } else {
            isSecond && isLeft
        }
    }
}

fn parseTimeField(inter: &'static str, min: u32, max: u32) -> Result<HashSet<u32>, CronError> {
    let mut points = HashSet::new();
    let parts: Vec<&'static str> = inter.split(",").collect();

    for part in parts {
        let x: Vec<&'static str> = part.split("/").collect();
        let y: Vec<&'static str> = x[0].split("-").collect();

        let mut _min = min;
        let mut _max = max;
        let mut step = 1u32;

        let (xLen, yLen) = (x.len(), y.len());

        if xLen == 1 && yLen == 1 {
            if y[0] != "*" {
                _min = try!(y[0].parse::<u32>());
                _max = _min;
            }
        } else if xLen == 1 && yLen == 2 {
            _min = try!(y[0].parse::<u32>());
            _max = try!(y[1].parse::<u32>());

        } else if xLen == 2 && yLen == 1 && x[0] == "*" {
            step = try!(x[1].parse::<u32>());

        } else {
            return Err(CronError::ParseFormat(part));
        }

        for i in (_min.._max + 1).filter(|x| x % step == 0).collect::<Vec<u32>>() {
            points.insert(i);
        }
    }

    Ok(points)
}

#[test]
fn test_parse_intervals() {
    assert!(Scheduler::new("*/2 1-8,11 * * *").is_ok());
    assert!(Scheduler::new("0 */2 1-8,11 * * *").is_ok());
    assert!(Scheduler::new("*/2 1-4,16,11,17 * * *").is_ok());
    assert!(Scheduler::new("05 */2 1-8,11 * * * *").is_err());
    assert!(Scheduler::new("05 */ 1-8,11 * * *").is_err());
}

fn main() {
    let cron01 = Scheduler::new("*/2 1-4,16,11,17 * * *").unwrap();
    let tm = time::now();

    println!("{:?}", cron01);
    println!("{:?} {}", &tm, cron01.isTimeUp(&tm));
}
