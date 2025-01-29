//! # Error Type Definitions for the [`HelpSystem`].
//!
//! This module defines the comprehensive [`Error`] type for the
//! Help System API.
//!
//! [`HelpSystem`]: super::HelpSystem

use thiserror::Error as ThisError;

/// Comprehensive error type for the help system.
#[derive(ThisError, Debug)]
pub enum Error {
    /// The user tried to look up a help topic that doesn't exist.
    #[error("couldn't find topic `{0}`. To list all topics, use `help:topics()`")]
    TopicNotFound(String),
}

impl Error {
    /// Create a new [`Error::TopicNotFound`]
    #[must_use]
    pub fn topic_not_found(name: &str) -> Self {
        Self::TopicNotFound(name.to_owned())
    }
}
