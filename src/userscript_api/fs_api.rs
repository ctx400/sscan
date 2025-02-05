//! # Ergonomic Filesystem APIs for Lua
//!
//! The [`FsApi`] provides userscript access to ergonomic file and
//! directory handling APIs. Stock Lua has no concept of filesystems or
//! directories, meaning that simple things - like recursively finding
//! files - requires clunky, non-portable solutions like this:
//!
//! ```lua
//! -- Recursively list files, Linux only
//! -- Windows requires a different implementation
//! local files = {}
//! do
//!   local handle <close> = io.popen'find . -type f'
//!   local output = handle:read'a'
//!   output:gsub([^\r\n], function(file) table.insert(files, file) end)
//! end
//! ```
//!
//! To remediate this deficiency, [`FsApi`] exposes several APIs to the
//! userscript environment to vastly improve the experience of dealing
//! with files in Lua.
//!
//! The API does not try to duplicate any existing Lua standard library
//! functionality. For example, there is no `fs:open()` method because
//! the stdlib already provides `io:open()`. Instead, it adds useful
//! concepts like directories, permission checking, and recursive
//! walking capabilities.
//!
//! ## Userscript API
//!
//! This is a userscript API. The API's functionality is registered with
//! the Lua virtual machine, where userscripts can call into it.
//!
//! For help, call `help 'fs'` from Lua, or see [`topics::fs`].
//!
//! [`topics::fs`]: crate::userscript_api::help_system::topics::fs

pub mod path_obj;
pub mod error;

use std::path::PathBuf;
use crate::userscript_api::{ApiObject, include::*};

/// # The Filesystem Manipulation API
///
/// The filesystem APIs expose methods and objects to Lua for handling
/// files and directories in a much more ergonomic manner than stock
/// Lua provides.
pub struct FsApi;

impl LuaUserData for FsApi {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Create a new PathObj to make use of the PathObj methods.
        methods.add_async_method("path", |_, _, path: PathBuf| async move {
            Ok(path_obj::PathObj(path))
        });

        // Test if a path is readable with current permissions.
        // Does not test if writable, and shouldn't anyways (TOCTOU!)
        //
        // ## Return Value
        // bool - True if the path is valid and readable.
        methods.add_async_method("test", |_, _, path: PathBuf| async move {
            let Ok(path) = path.canonicalize() else { return Ok(false) };
            if path.is_dir() {
                return Ok(path.read_dir().is_ok());
            }
            if path.is_symlink() {
                return Ok(path.read_link().is_ok());
            }
            if path.is_file() {
                return Ok(std::fs::File::open(path).is_ok());
            }
            Ok(true)
        });

        // List the contents of a directory.
        //
        // ## Return Value
        // Vec<PathObj> - Userdata with fields for ergonomic property access.
        //
        // ## Errors
        // - The provided path is not a directory.
        // - Cannot access the directory.
        methods.add_async_method("listdir", |_, _, path: PathBuf| async move {
            // Stores PathObj items to return to Lua.
            let mut subpaths: Vec<path_obj::PathObj> = Vec::with_capacity(1024);

            // Iterate over the directory to get a list of files.
            let path: PathBuf = path.canonicalize().map_err(|source| error::Error::InvalidPath { path, source })?;
            for entry in path.read_dir().map_err(|source| error::Error::ReadDirError { path: path.clone(), source })? {
                let Ok(entry) = entry else { continue };
                subpaths.push(path_obj::PathObj(entry.path()));
            }

            // Return the PathObj items to Lua.
            subpaths.shrink_to_fit();
            Ok(subpaths)
        });
    }
}

impl ApiObject for FsApi {
    fn name(&self) -> &'static str {
        "fs"
    }
}
