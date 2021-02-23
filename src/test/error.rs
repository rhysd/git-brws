use crate::error::{Error, ErrorKind, ExpectedNumberOfArgs};
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
struct DummyError;
impl fmt::Display for DummyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dummy")
    }
}

impl error::Error for DummyError {}

fn dummy_io_error() -> Box<Error> {
    let inner = DummyError;
    let io_err = io::Error::new(io::ErrorKind::Other, inner);
    Error::new(ErrorKind::IoError(io_err))
}

#[test]
fn error_message_formating() {
    let err = Error::new(ErrorKind::WrongNumberOfArgs {
        expected: ExpectedNumberOfArgs::Range(0, 1),
        actual: 2,
        kind: "diff".to_string(),
    });

    assert!(matches!(err.kind(), ErrorKind::WrongNumberOfArgs { .. }));

    let msg = format!("{}", err);
    assert!(msg.contains("Invalid number of arguments"), "{:?}", msg);
}

#[test]
fn print_error_to_stderr() {
    dummy_io_error().eprintln();
}

#[test]
fn inner_error_as_source() {
    use error::Error;
    let err = crate::error::Error::new(ErrorKind::DiffDotsNotFound);
    assert!(err.source().is_none());

    let err = dummy_io_error();
    let inner = err.source().unwrap();
    assert!(format!("{}", inner).contains("dummy"), "{:?}", inner);
}
