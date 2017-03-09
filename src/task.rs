use std::io::prelude::*;
use std::process::{Command, Child, Stdio};
use std::sync::Mutex;
use std::sync::mpsc::{self, Sender, Receiver};
use std::default::Default;
use std::thread;
use std::io::BufReader;
use std::iter::Iterator;
use libc;

use error::Error;

pub struct Task {
    command: Command,
    process: Option<Child>,

    isStarted: bool,
    pid: Option<u32>,

    outputChanRx: Option<Receiver<String>>,
    outputChanTx: Option<Sender<String>>,
}

impl Task {
    pub fn new(cmd: Command) -> Task {
        Task {
            command: cmd,
            process: None,
            isStarted: false,
            outputChanTx: None,
            outputChanRx: None,
            pid: None,
        }
    }

    // Start the specified command but
    // does not wait for it to complete.
    pub fn start(&mut self) -> Result<(), Error> {
        let (outputChanTx, outputChanRx) = mpsc::channel();
        let tx = outputChanTx.clone();

        let mut process = try!(self.command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| Error::ErrRunCommand(e.to_string())));

        unsafe {
            let pid = process.id() as libc::pid_t;
            libc::setpgid(pid, pid);
        }

        let stdout = process.stdout.take().unwrap();

        self.isStarted = true;
        self.process = Some(process);
        self.outputChanTx = Some(outputChanTx);
        self.outputChanRx = Some(outputChanRx);


        thread::spawn(move || {
            let breader = BufReader::new(stdout);

            for line in breader.lines() {
                if let Ok(mut x) = line {
                    tx.send(x);
                }
            }
        });

        Ok(())
    }

    // Start the specified command and
    // waits for it to complete.
    pub fn run(&mut self) {}

    pub fn kill(&mut self) -> Result<(), Error> {
        if let Some(ref mut process) = self.process {
            process.kill()
                .map_err(|e| Error::ErrKillProcess(e.to_string()))
                .map(|_| ())
        } else {
            Ok(())
        }
    }

    pub fn packets(&mut self) -> TaskLogIterator {
        TaskLogIterator { task: self }
    }
}

pub struct TaskLogIterator<'a> {
    task: &'a mut Task,
}

impl<'a> Iterator for TaskLogIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.task.outputChanRx {
            Some(ref x) => x.try_recv().ok(),
            None => None,
        }
    }
}

#[test]
fn test_start_task() {
    let mut cmd = Command::new("ls");
    cmd.arg("-al")
        .arg(".");

    let mut task = Task::new(cmd);
    task.start();

    for line in task.packets() {
        println!("==> {:?}", line);
    }
}
