# cron-rs

Cron library that is written in rust.

## USAGE

### Cron Time Format 

`[SECOND] MINUTE HOUR DAY MONTH WEEKDAY`

* SECOND (optional): second, 0-59.
* MINUTE (required): minute, 0-59.
* HOUR (required): hour, 0-23.
* DAY (required): day of month, 1-31.
* MONTH (required): month of year, 1-12.
* WEEKDAY (required): day of week, 0-6.

#### Good Examples 
`*/2 1-8,11 * * *`

`00 */2 1-8,11 * * *`

### Make A Scheduler
Scheduler is a cron time scheduler. 

`
    let sch = Scheduler::new("*/2 1-8,11 * * *").unwrap();
`

### Verify Time 

` 
    let tm = time::now();
    assert!(sch.isTimeUP(&tm));
` 

## DEPENDENICIES 

* time
* regex

## FAQ

