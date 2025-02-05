//! # Error type definitions for [`FsApi`]
//!
//! This module defines the comprehensive error type for the sscan
//! filesystem APIs. Any errors returned from the [`FsApi`] or a
//! [`PathObj`] will be of this type.
//!
//! [`FsApi`]: super::FsApi
//! [`PathObj`]: super::path_obj::PathObj

use std::path::PathBuf;
use thiserror::Error as ThisError;
use crate::userscript_api::include::*;

/// Comprehensive error type for FsApi
#[derive(ThisError, Debug)]
pub enum Error {
    /// The provided path is invalid.
    #[error("invalid path `{path}`: {source}")]
    InvalidPath {
        /// The erroneous path that was provided.
        path: PathBuf,

        /// Inner IO error that occurred.
        source: std::io::Error,
    },

    /// Unable to read the requested dir.
    #[error("failed to list dir {}: {source}", path.to_string_lossy())]
    ReadDirError {
        /// Path to the directory
        path: PathBuf,

        /// Inner IO error that occurred.
        source: std::io::Error,
    }
}

impl From<Error> for LuaError {
    fn from(value: Error) -> Self {
        value.into_lua_err()
    }
}
