use std::process::{Command, Child};
use std::sync::Mutex;
use std::sync::mpsc::{self, Sender, Receiver};
use std::default::Default;

use error::Error;

pub struct Task {
    command: Command,

    isStarted: bool,

    stopChan: Option<Mutex<Receiver<()>>>,
    outputChan: Option<Mutex<Sender<()>>>,
}

impl Task {
    pub fn new(cmd: Command) -> Task {
        Task {
            command: cmd,
            isStarted: false,
            stopChan: None,
            outputChan: None,
        }
    }

    // Start the specified command but
    // does not wait for it to complete.
    pub fn start(&mut self) -> (Result<(Sender<()>, Receiver<()>), Error>) {
        let (stopChanTx, stopChanRx) = mpsc::channel::<()>();
        let (outputChanTx, outputChanRx) = mpsc::channel::<()>();

        self.isStarted = true;
        self.stopChan = Some(Mutex::new(stopChanRx));
        self.outputChan = Some(Mutex::new(outputChanTx));

        Ok((stopChanTx, outputChanRx))
    }

    // Start the specified command and
    // waits for it to complete.
    pub fn run() {}

    pub fn stop(&mut self) {}
}
