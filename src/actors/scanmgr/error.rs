//! # Error Type Definitions for [`ScanMgr`]
//!
//! This module defines the comprehensive error type for the scan
//! manager service. Any error arising from interacting with the service
//! will be of this type.
//!
//! [`ScanMgr`]: super::ScanMgr

use thiserror::Error as ThisError;

/// Type alias for fallible return types that may return [`Error`].
pub type ScanMgrResult<T> = Result<T, Error>;

/// Comprehensive error type for [`ScanMgr`]
///
/// [`ScanMgr`]: super::ScanMgr
#[derive(ThisError, Debug)]
pub enum Error {
    /// The Lua virtual machine is not running
    #[error("the lua virtual machine is not running")]
    NoLuaVm,

    /// The global scan queue service is not running
    #[error("the global scan queue service is not running")]
    NoQueue,

    /// The user engine service is not running
    #[error("the userscript scan engine service is not running")]
    NoUserEngine,

    /// The scan manager service is not running
    #[error("the scan manager service is not running")]
    NoScanMgr,
}
