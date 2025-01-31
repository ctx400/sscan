//! # Error Type Definitions for [`LuaVM`]
//!
//! This module defines the comprehensive [`Error`] type for any
//! errors encountered when processing incoming messages.
//!
//! [`LuaVM`]: super::LuaVM

use thiserror::Error as ThisError;

/// Result type alias for [`LuaVM`](super::LuaVM)
pub type LuaVmResult<T> = Result<T, Error>;

/// Comprehensive error type for the Lua virtual machine.
#[derive(ThisError, Debug)]
pub enum Error {
    /// An internal Lua interpreter error occurred.
    #[error("the Lua interpreter encountered an error: {source}")]
    InternalLuaError {
        /// The inner Lua error that occurred.
        source: mlua::Error,
    },
}

impl From<mlua::Error> for Error {
    fn from(value: mlua::Error) -> Self {
        Self::InternalLuaError { source: value }
    }
}
