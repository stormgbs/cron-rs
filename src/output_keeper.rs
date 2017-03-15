use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

use error::Error;
use server::Server;
use output_chunk::Message;

pub trait OutputHandler {
    type Err;
    fn push(&self, Message) -> Result<(), Self::Err>;
}

pub struct OutputKeeper<'a> {
    server: &'a Server,

    pub tx: Sender<Message>,
    rx: Option<Receiver<Message>>,
}

impl<'a> OutputKeeper<'a> {
    pub fn new(srv: &'a Server) -> OutputKeeper {
        let (tx, rx) = mpsc::channel();

        let mut keeper = OutputKeeper {
            server: srv,
            rx: Some(rx),
            tx: tx,
        };

        keeper.start();
        keeper
    }

    fn start(&mut self) {
        let rx = self.rx.take().unwrap();

        thread::spawn(move || loop {
            match rx.recv() {
                Ok(msg) => {
                    println!("received message: {:?}", msg.body);
                    //TODO
                    //push message
                }
                Err(e) => {
                    println!("message keeper recv error: {:?}", e);
                }
            }
        });

        // do not join it
    }
}

impl<'a> OutputHandler for OutputKeeper<'a> {
    type Err = Error;

    fn push(&self, msg: Message) -> Result<(), Error> {
        //TODO
        Ok(())
    }
}
