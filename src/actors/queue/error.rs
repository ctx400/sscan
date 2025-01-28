//! # Error Type Definitions for [`Queue`]
//!
//! This module defines the comprehensive [`Error`] type for any errors
//! encountered when processing incoming messages.
//!
//! [`Queue`]: super::Queue

use thiserror::Error as ThisError;

/// Type alias for any result that might return [`Error`].
pub type QueueResult<T> = Result<T, Error>;

/// # Comprehensive error type for [`Queue`]
///
/// This enum defines all types of errors that can occur with [`Queue`]
/// operations or any operation on a [`DataItem`].
///
/// [`Queue`]: super::Queue
/// [`DataItem`]: super::data_item::DataItem
#[derive(ThisError, Debug)]
pub enum Error {
    /// An IO-related error occurred (filesystem, permissions, etc.)
    #[error("an IO error occurred: {source}")]
    IOError {
        /// The inner IO error causing the failure.
        source: std::io::Error,
    },

    /// A [`Dequeue`] message was sent to an empty [`Queue`].
    ///
    /// [`Dequeue`]: super::messages::Dequeue
    /// [`Queue`]: super::Queue
    #[error("the item queue is empty")]
    QueueEmpty,

    /// Unable to send a message to the queue actor.
    #[error("unable to communicate with the global queue")]
    SendError,

    /// The [`Queue`] actor is not currently running.
    ///
    /// [`Queue`]: super::Queue
    #[error("there is no running global queue")]
    NoGlobalQueue,

    /// The Lua userscript environment is not running.
    #[error("the Lua userscript environment does not appear to be running")]
    NoLuaVm,
}

impl Error {
    /// Creates a new [`Error::QueueEmpty`].
    #[must_use]
    pub fn empty() -> Self {
        Self::QueueEmpty
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Self::IOError { source }
    }
}
