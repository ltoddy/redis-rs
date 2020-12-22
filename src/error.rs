use std::borrow::Cow;

/// An enum of all error kinds.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ErrorKind {
    ResponseError,
    AuthenticationFailed,
    TypeError,
    Io,
    ClientError,
    FromServer,
}

enum Repr {
    Io(std::io::Error),
    Custom(ErrorKind, Cow<'static, str>),
}

pub struct RedisError {
    repr: Repr,
}

impl std::fmt::Display for RedisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.repr {
            Repr::Io(ref e) => e.fmt(f),
            Repr::Custom(_, ref desc) => write!(f, "{}", desc),
        }
    }
}

impl std::fmt::Debug for RedisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for RedisError {}

impl From<std::io::Error> for RedisError {
    fn from(e: std::io::Error) -> Self {
        RedisError { repr: Repr::Io(e) }
    }
}

impl From<std::num::ParseIntError> for RedisError {
    fn from(_: std::num::ParseIntError) -> Self {
        RedisError {
            repr: Repr::Custom(ErrorKind::TypeError, Cow::from("invalid digit")),
        }
    }
}

impl From<std::num::ParseFloatError> for RedisError {
    fn from(_: std::num::ParseFloatError) -> Self {
        RedisError {
            repr: Repr::Custom(ErrorKind::TypeError, Cow::from("invalid float")),
        }
    }
}

impl From<std::string::FromUtf8Error> for RedisError {
    fn from(_: std::string::FromUtf8Error) -> Self {
        RedisError {
            repr: Repr::Custom(ErrorKind::TypeError, Cow::from("invalid utf-8")),
        }
    }
}

impl From<std::str::Utf8Error> for RedisError {
    fn from(_: std::str::Utf8Error) -> Self {
        RedisError {
            repr: Repr::Custom(ErrorKind::TypeError, Cow::from("invalid utf-8")),
        }
    }
}

impl RedisError {
    pub fn custom<S: ToString>(kind: ErrorKind, desc: S) -> RedisError {
        RedisError {
            repr: Repr::Custom(kind, Cow::from(desc.to_string())),
        }
    }
}

impl RedisError {
    pub fn kind(&self) -> ErrorKind {
        match self.repr {
            Repr::Io(_) => ErrorKind::Io,
            Repr::Custom(kind, _) => kind,
        }
    }

    pub fn is_io_error(&self) -> bool {
        self.as_io_error().is_some()
    }

    pub fn as_io_error(&self) -> Option<&std::io::Error> {
        match self.repr {
            Repr::Io(ref e) => Some(e),
            _ => None,
        }
    }

    pub fn is_connection_refuse(&self) -> bool {
        match self.repr {
            Repr::Io(ref e) => matches!(
                e.kind(),
                std::io::ErrorKind::ConnectionRefused | std::io::ErrorKind::NotFound
            ),
            _ => false,
        }
    }

    pub fn is_timeout(&self) -> bool {
        match self.repr {
            Repr::Io(ref e) => matches!(e.kind(), std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock),
            _ => false,
        }
    }

    pub fn is_connection_dropped(&self) -> bool {
        match self.repr {
            Repr::Io(ref e) => matches!(
                e.kind(),
                std::io::ErrorKind::ConnectionReset | std::io::ErrorKind::BrokenPipe
            ),
            _ => false,
        }
    }
}
