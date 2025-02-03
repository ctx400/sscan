//! # Error Type Definitions for [`UserEngine`]
//!
//! This module defines the comprehensive error type for the userscript
//! scan engine service. All errors originating from the [`UserEngine`]
//! are of this type.
//!
//! [`UserEngine`]: super::UserEngine

use thiserror::Error as ThisError;

/// Type alias for results that may be [`Error`]
pub type UserEngineResult<T> = Result<T, Error>;

/// Comprehensive error type for [`UserEngine`].
///
/// [`UserEngine`]: super::UserEngine
#[derive(ThisError, Debug)]
pub enum Error {
    /// [`LuaVM`] was not running at the time of invocation.
    ///
    /// [`LuaVM`]: crate::actors::lua_vm::LuaVM
    #[error("the lua userscript environment is not running")]
    NoLuaVm,

    /// [`UserEngine`] was not running at the time of invocation.
    ///
    /// [`UserEngine`]: super::UserEngine
    #[error("the userscript scan engine service is not running")]
    NoUserEngine,

    /// An error occurred trying to invoke a userscript scan engine.
    #[error("failed to invoke userscript engine {engine}: {source}")]
    EngineInvocation {
        /// Name of the userscript scan engine that failed.
        engine: String,

        /// Inner Lua error for more context.
        source: mlua::Error,
    },
}

impl Error {
    /// Create a new [`Error::EngineInvocation`].
    #[must_use]
    pub fn engine_invocation(engine: String, source: mlua::Error) -> Self {
        Self::EngineInvocation { engine, source }
    }
}
