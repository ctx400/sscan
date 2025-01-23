//! # Error Type Definitions for the [`HelpSystem`].
//!
//! This module defines the comprehensive [`HelpError`] type for the
//! Help System API.
//!
//! [`HelpSystem`]: super::HelpSystem

use thiserror::Error;

/// Comprehensive error type for the help system.
#[derive(Error, Debug)]
pub enum HelpError {
    /// The user tried to use a reserved topic name, such as `topics`.
    #[error("cannot add help topic: use of reserved topic name `{0}`")]
    ReservedTopicName(String),

    /// The user tried to look up a help topic that doesn't exist.
    #[error("couldn't find topic `{0}`. To list all topics, use `help:topics()`")]
    TopicNotFound(String),
}

impl HelpError {
    /// Create a new [`HelpError::ReservedTopicName`]
    pub fn reserved_topic_name(name: &str) -> Self {
        Self::ReservedTopicName(name.to_owned())
    }

    /// Create a new [`HelpError::TopicNotFound`]
    pub fn topic_not_found(name: &str) -> Self {
        Self::TopicNotFound(name.to_owned())
    }
}
