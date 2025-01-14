//! # Userscript APIs
//!
//! This module and its submodules provide the userscript environment
//! with APIs for interacting with sscan programmatically.
//!

// Submodules containing userscript APIs
pub mod version_info;

// Re-exports of key API functions.
pub(super) use version_info::register_version_apis;
