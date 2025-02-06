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
use crate::userscript_api::{ApiObject, include::{LuaEither, LuaExternalError, LuaUserData, LuaUserDataMethods, LuaUserDataRef}, fs_api::{path_obj::PathObj, error::Error}};

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
            Ok(PathObj(path))
        });

        // Test if a path is readable with current permissions.
        // Does not test if writable, and shouldn't anyways (TOCTOU!)
        //
        // ## Return Value
        // bool - True if the path is valid and readable.
        methods.add_async_method("test", |_, _, path: LuaEither<PathBuf, LuaUserDataRef<PathObj>>| async move {
            let path: PathBuf = match path {
                LuaEither::Left(pb) => pb,
                LuaEither::Right(po) => po.0.clone(),
            };

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
        methods.add_async_method("listdir", |_, _, path: LuaEither<PathBuf, LuaUserDataRef<PathObj>>| async move {
            let path: PathBuf = match path {
                LuaEither::Left(pb) => pb,
                LuaEither::Right(po) => po.0.clone(),
            };

            // Validate an actual directory was passed.
            if !path.is_dir() {
                return Err(Error::NotADirectory { path }.into_lua_err());
            }

            // Stores PathObj items to return to Lua.
            let mut subpaths: Vec<PathObj> = Vec::with_capacity(1024);

            // Iterate over the directory to get a list of files.
            let path: PathBuf = path.canonicalize().map_err(|source| error::Error::InvalidPath { path, source })?;
            for entry in path.read_dir().map_err(|source| error::Error::ReadDirError { path: path.clone(), source })? {
                let Ok(entry) = entry else { continue };
                subpaths.push(PathObj(entry.path()));
            }

            // Return the PathObj items to Lua.
            subpaths.shrink_to_fit();
            Ok(subpaths)
        });

        // List all filesystem objects recursively.
        //
        // This function is iterative rather than recursive, due to the
        // constraints of futures and async-await programming. It
        // *could* be done recursively, but it's a hassle with async.
        //
        // Because of the iterative nature of this function, there may
        // be duplicate paths. As such, the vector is sorted and deduped
        // before returning to Lua.
        methods.add_async_method("walk", |_, _, basepath: LuaEither<PathBuf, LuaUserDataRef<PathObj>>| async move {
            let basepath: PathBuf = match basepath {
                LuaEither::Left(pb) => pb,
                LuaEither::Right(po) => po.0.clone(),
            };

            // Validate an actual directory was passed.
            if !basepath.is_dir() {
                return Err(Error::NotADirectory { path: basepath }.into_lua_err());
            }

            // Set up the iteration and result vectors.
            let mut dirq: Vec<PathBuf> = Vec::with_capacity(16384);
            let mut path_objs: Vec<PathObj> = Vec::with_capacity(16384);
            dirq.push(basepath.canonicalize()?);

            // Walk all subdirectories
            while let Some(current_dir) = dirq.pop() {
                path_objs.push(PathObj(current_dir.clone()));

                // Skip directory if unreadable.
                if let Ok(dir_reader) = current_dir.read_dir() {
                    for entry in dir_reader {
                        // Skip entries within a dir if they are unreadable.
                        let Ok(entry) = entry else { continue; };
                        let path: PathBuf = entry.path();

                        // Push new dirs onto the queue
                        // But skip symlinks to avoid infinite loops.
                        if !path.is_symlink() && path.is_dir() {
                            dirq.push(path.clone());
                        }

                        // Push a new path object into the results vec
                        path_objs.push(PathObj(path));
                    }
                }
            }

            // Sort and dedupe the list of path objects.
            path_objs.sort();
            path_objs.dedup_by(|a: &mut PathObj, b: &mut PathObj| a.0.eq(&b.0));

            // Return the path objects to Lua.
            path_objs.shrink_to_fit();
            Ok(path_objs)
        });
    }
}

impl ApiObject for FsApi {
    fn name(&self) -> &'static str {
        "fs"
    }
}
