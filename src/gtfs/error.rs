use std::error::Error as StdError;
use std::fmt;
use csv::{DeserializeErrorKind, Error as CsvError, ErrorKind};

#[derive(Debug)]
pub enum Error {
    Feed(String),
    Csv(String, CsvError),
    FieldError(String, u64, DeserializeErrorKind, Option<String>),
    LineError(String, ErrorKind),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Feed(ref message) => write!(f, "error in feed - {}", message),
            Error::Csv(ref filename, ref err) => {
                write!(f, "error parsing {} - {}", filename, format!("{}", err))
            }
            Error::FieldError(ref filename, ref lineno, ref errk, ref field) => match *field {
                Some(ref fieldname) => write!(
                    f,
                    "error parsing {} in {}:{} - {}",
                    fieldname,
                    filename,
                    lineno,
                    format!("{}", errk)
                ),
                None => write!(
                    f,
                    "error parsing {}:{} - {}",
                    filename,
                    lineno,
                    format!("{}", errk)
                ),
            },
            Error::LineError(ref filename, ref err) => match *err {
                // TODO: Find out when position can be None
                ErrorKind::UnequalLengths {
                    ref pos,
                    ref expected_len,
                    ref len,
                } => write!(
                    f,
                    "error parsing {}:{} - expected {} fields but got {} fields",
                    filename,
                    pos.as_ref().unwrap().line(),
                    expected_len,
                    len
                ),
                ErrorKind::Utf8 { ref pos, ref err } => write!(
                    f,
                    "error parsing {}:{} - {:?}",
                    filename,
                    pos.as_ref().unwrap().line(),
                    err
                ),
                _ => write!(f, "error parsing {} - {:?}", filename, err),
            },
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            Feed(..) => "error using feed",
            Csv(_, ref err) => err.description(),
            FieldError(..) => "error processing field of a line",
            LineError(..) => "error processing line",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        use self::Error::*;
        match *self {
            Feed(..) => None,
            Csv(_, ref err) => Some(err),
            // ErrorKinds don't implement std::error::Error, need to keep original error
            FieldError(..) => None,
            LineError(..) => None,
        }
    }
}
