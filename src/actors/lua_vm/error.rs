//! # Error Type Definitions for [`LuaVM`]
//!
//! This module defines the comprehensive [`LuaVmError`] type for any
//! errors encountered when processing incoming messages.
//!
//! [`LuaVM`]: super::LuaVM

use thiserror::Error;

/// Comprehensive error type for the Lua virtual machine.
#[derive(Error, Debug)]
pub enum LuaVmError {
    /// An internal Lua interpreter error occurred.
    #[error("the Lua interpreter encountered an error: {source}")]
    InternalLuaError { source: mlua::Error },
}

impl From<mlua::Error> for LuaVmError {
    fn from(value: mlua::Error) -> Self {
        Self::InternalLuaError { source: value }
    }
}
