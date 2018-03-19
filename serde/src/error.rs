use std;
use std::fmt::{self, Display};

use serde::{ser, de};


/// Result type for serialization.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for serialization.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
	Message(String),
	Unimplemented,
	UnsupportedType,
	ExpectedType(String),
}

impl ser::Error for Error {
	fn custom<T: Display>(msg: T) -> Self {
		Error::Message(msg.to_string())
	}
}

impl de::Error for Error {
	fn custom<T: Display>(msg: T) -> Self {
		Error::Message(msg.to_string())
	}
}

impl std::error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::Message(ref msg) => msg,
			_ => "dunno",
		}
	}
}

impl Display for Error {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str(std::error::Error::description(self))
	}
}
