use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Weak;

use task::Task;

pub struct TaskMaster<'a, 'b: 'a> {
    tasks: RefCell<HashMap<u32, Task<'a, 'b>>>,
}
