use std::process::{Command, Child};
use std::sync::Mutex;
use std::sync::mpsc::{self, Sender, Receiver};

pub struct Task {
    command: Command,

    stopChan: Mutex<Receiver<()>>,
    outputChan: Mutex<Receiver<()>>,
}

impl Task {
    pub fn start(&mut self) -> () {
        ()
    }
    pub fn stop(&mut self) {}
}
