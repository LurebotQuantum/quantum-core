
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorKind {
    IoError(std::io::Error),
    Other(String),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new<T>(kind: T) -> Error
        where T: Into<ErrorKind> {
        Error {
            kind: kind.into()
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::IoError(err) => {
                err.fmt(f)
            },
            ErrorKind::Other(message) => {
                write!(f, "Quantum Error: {}", message)
            },
        }
    }
}

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(message: &str) -> Error {
        Error {
            kind: ErrorKind::Other(message.to_owned())
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Error {
        Error {
            kind: ErrorKind::Other(message)
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error {
            kind: ErrorKind::IoError(err)
        }
    }
}
