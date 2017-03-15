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
use output_chunk::{Message, OutputChunkIterator};

pub struct Task {
    id: u32,
    command: Command,
    process: Option<Child>,

    isStarted: bool,
    pid: Option<u32>,
}

impl Task {
    pub fn new(id: u32, cmd: Command) -> Task {
        Task {
            id: id,
            command: cmd,
            process: None,
            isStarted: false,
            pid: None,
        }
    }

    // Start the specified command but
    // does not wait for it to complete.
    pub fn start(&mut self, output: Sender<Message>) -> Result<(), Error> {
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


        thread::spawn(move || {
            let breader = BufReader::new(stdout);
            let outputIter = OutputChunkIterator::new(breader);

            for chunk in outputIter {
                if let Ok(mut x) = chunk {
                    output.send(Message {
                        taskId: 100,
                        //taskId: self.id,
                        kind: "unknown".to_string(),
                        startUnixTimeNs: 0,
                        endUnixTimeNs: 0,
                        body: "WTH".to_string(),
                    });
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

    //pub fn packets(&mut self) -> TaskLogIterator {
    //    TaskLogIterator { task: self }
    //}
}

//pub struct TaskLogIterator<'a> {
//    task: &'a mut Task,
//}
//
//impl<'a> Iterator for TaskLogIterator<'a> {
//    type Item = String;
//
//    fn next(&mut self) -> Option<String> {
//        match self.task.outputChanRx {
//            Some(ref x) => {
//                if let Ok(mut msg) = x.try_recv() {
//                    msg.take()
//                } else {
//                    None
//                }
//            }
//            None => None,
//        }
//    }
//}

#[test]
fn test_start_task() {
    let mut cmd = Command::new("ls");
    cmd.arg("-al")
        .arg(".");

    let mut task = Task::new(cmd);
    task.start();
}
