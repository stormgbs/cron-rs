use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Weak;

use job::Job;

pub struct JobMaster {
    jobs: RefCell<HashMap<u32, Job>>,
}
