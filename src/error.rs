use std::num::ParseIntError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ErrCronFormat(String),
    ErrParseJob(String),
    ErrParseInt(ParseIntError),
    ErrRunCommand(String),
    ErrKillProcess(String),
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::ErrParseInt(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::ErrCronFormat(ref x) => write!(f, "<ErrCronFormat> {:?}", x),
            &Error::ErrParseJob(ref x) => write!(f, "<ErrParseJob> {:?}", x),
            &Error::ErrRunCommand(ref x) => write!(f, "<ErrRunCommand> {:?}", x),
            &Error::ErrKillProcess(ref x) => write!(f, "<ErrKillProcess> {:?}", x),
            &Error::ErrParseInt(ref e) => write!(f, "<ErrParseInt> {:?}", e),
        }
    }
}
