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
use output_keeper::OutputKeeper;

pub struct Job<'a, 'b: 'a> {
    id: u64,
    command: Command,
    process: Option<Child>,

    isStarted: bool,
    pid: Option<u32>,

    keeper: &'a OutputKeeper<'b>,
}

impl<'a, 'b: 'a> Job<'a, 'b> {
    pub fn new(cmd: Command, keeper: &'b OutputKeeper) -> Job<'a, 'b> {
        Job {
            id: Default::default(),
            command: cmd,
            process: None,
            isStarted: false,
            pid: None,
            keeper: keeper,
        }
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = id
    }

    // Start the command specified in background, and
    // does not wait for it to complete.
    pub fn start(&mut self) -> Result<(), Error> {
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

        let output = self.keeper.tx.clone();

        let jobId = self.id;

        thread::spawn(move || {
            let breader = BufReader::new(stdout);
            let outputIter = OutputChunkIterator::new(breader);

            for chunk in outputIter {
                if let Ok(mut x) = chunk {
                    output.send(Message {
                        jobId: jobId,
                        kind: "unknown".to_string(),
                        startUnixTimeNs: 0,
                        endUnixTimeNs: 0,
                        body: x.clone(),
                    });
                }
            }
        });

        Ok(())
    }

    // Start the specified command and
    // wait for it to be completed.
    pub fn run(&mut self) {
        //TODO
    }

    pub fn kill(&mut self) -> Result<(), Error> {
        if let Some(ref mut process) = self.process {
            process.kill()
                .map_err(|e| Error::ErrKillProcess(e.to_string()))
                .map(|_| ())
        } else {
            Ok(())
        }
    }

    //pub fn packets(&mut self) -> JobLogIterator {
    //    JobLogIterator { job: self }
    //}
}

//pub struct JobLogIterator<'a> {
//    job: &'a mut Job,
//}
//
//impl<'a> Iterator for JobLogIterator<'a> {
//    type Item = String;
//
//    fn next(&mut self) -> Option<String> {
//        match self.job.outputChanRx {
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
fn test_start_job() {
    let mut cmd = Command::new("ls");
    cmd.arg("-al")
        .arg(".");

    let mut job = Job::new(cmd);
    job.start();
}
