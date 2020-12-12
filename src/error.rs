#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Resp,

    ConnectionPoolClosed,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{}", e),
            Error::Resp => write!(f, "wrong Redis serialization protocol format"),
            Error::ConnectionPoolClosed => write!(f, "connection pool closed"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
