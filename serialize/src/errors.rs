//! Serializer errors.

#[derive(Fail, Debug)]
enum InnerError {
    /// Standard IO error.
    #[fail(display = "i/o error: {:?}", _0)]
    Io(#[cause] std::io::Error),

    /// Standard format error.
    #[fail(display = "fmt error: {:?}", _0)]
    Fmt(#[cause] std::fmt::Error),

    /// Custom error.
    #[fail(display = "serde: {:?}", _0)]
    Custom(String),
}

#[derive(Debug)]
pub struct Error {
    inner: InnerError,
}

impl std::error::Error for Error {}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error {
            inner: InnerError::Custom(msg.to_string()),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

macro_rules! err_converter {
    ( $a:ident, $b:ty ) => {
        impl From<$b> for InnerError {
            fn from(e: $b) -> Self {
                InnerError::$a(e)
            }
        }

        impl From<$b> for Error {
            fn from(e: $b) -> Self {
                Error { inner: e.into() }
            }
        }
    };
}

pub type Result<T> = std::result::Result<T, Error>;

err_converter!(Io, std::io::Error);
err_converter!(Fmt, std::fmt::Error);
