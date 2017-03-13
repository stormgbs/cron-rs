use std::io::prelude::*;
use error::Error;
use time;
use std::time as stdtime;
use std::thread;

const MAXREADBUFFERSIZE: usize = 65536;

pub struct OutputIterator<B> {
    buf: B,
}

impl<B: BufRead> OutputIterator<B> {
    pub fn new(buf: B) -> OutputIterator<B> {
        OutputIterator { buf: buf }
    }
}

impl<B: BufRead> Iterator for OutputIterator<B> {
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Result<String, Error>> {
        let mut chunk = String::new();
        let mut buffer = [0; MAXREADBUFFERSIZE];

        loop {
            // job will be executed in the zero seconds,
            // so, just take easy :)
            if time::now().tm_sec == 59 {
                break;
            } else {
                match self.buf.read(&mut buffer[..]) {
                    Ok(0) => break,
                    Ok(n) => chunk.push_str(&String::from_utf8_lossy(&buffer[..n])), 
                    Err(e) => return Some(Err(Error::ErrRead(e.to_string()))),
                }

                thread::sleep(stdtime::Duration::from_millis(300))
            }
        }

        if chunk.len() == 0 {
            None
        } else {
            Some(Ok(chunk))
        }
    }
}
