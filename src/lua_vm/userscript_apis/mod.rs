//! # Userscript APIs
//!
//! Each submodule exposes a certain set of sscan APIs and data items to
//! the userscript environment to enable programmatic access and
//! configuration of sscan.
//!
//! See each submodule for userscript API documentation.
//!

// Submodules containing userscript APIs
pub mod version_info;

// Re-exports of key API functions.
pub(super) use version_info::register_version_apis;
