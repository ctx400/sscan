//! # Error Type Definitions for [`Queue`]
//!
//! This module defines the comprehensive [`Error`] type for any errors
//! encountered when processing incoming messages.
//!
//! [`Queue`]: super::Queue

use thiserror::Error as ThisError;

pub type QueueResult<T> = Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("an IO error occurred: {source}")]
    IOError {
        source: std::io::Error,
    },
    #[error("the item queue is empty")]
    QueueEmpty,
}

impl Error {
    pub fn empty() -> Self {
        Self::QueueEmpty
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError { source: value }
    }
}
