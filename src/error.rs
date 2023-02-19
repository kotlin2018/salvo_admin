use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Default(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Default(error) => write!(f, "{}", error),
        }
    }
}

impl From<&str> for Error{
    fn from(arg: &str) -> Self {
        return Error::Default(arg.to_string());
    }
}