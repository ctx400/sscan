//! # An ergonomic path object.
//!
//! The [`PathObj`] is a Lua userdata type that encapsulates a
//! [`PathBuf`], providing fields and API methods to ergonomically
//! interact with the path. While not perfect, the path object is
//! far superior to stock Lua's path handling, which involves
//! dealing with raw strings.
//!
//! See [`topics::path`] to learn how to use [`PathObj`].
//!
//! [`topics::path`]: crate::userscript_api::help_system::topics::path

use std::path::PathBuf;
use crate::userscript_api::{include::*, fs_api::error::Error};

/// Represents a Directory Entry
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct PathObj(pub PathBuf);

impl LuaUserData for PathObj {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        // The PathObj's full path.
        fields.add_field_method_get("path", |_, this: &PathObj| {
            Ok(this.0.clone())
        });

        // Filename of the PathObj.
        fields.add_field_method_get("name", |lua: &Lua, this: &PathObj| {
            let Some(filename) = this.0.file_name() else { return Ok(LuaNil) };
            let filename: LuaValue = filename.into_lua(lua)?;
            if filename.is_string() {
                Ok(filename)
            } else {
                Ok(LuaNil)
            }
        });

        // File extension of the PathObj.
        fields.add_field_method_get("ext", |lua: &Lua, this: &PathObj| {
            let Some(extension) = this.0.extension() else { return Ok(LuaNil) };
            let extension: LuaValue = extension.into_lua(lua)?;
            if extension.is_string() {
                Ok(extension)
            } else {
                Ok(LuaNil)
            }
        });

        // The file stem (filename without extension) of the PathObj.
        fields.add_field_method_get("stem", |lua: &Lua, this: &PathObj| {
            let Some(stem) = this.0.file_stem() else { return Ok(LuaNil) };
            let stem: LuaValue = stem.into_lua(lua)?;
            if stem.is_string() {
                Ok(stem)
            } else {
                Ok(LuaNil)
            }
        });

        // The parent PathObj
        fields.add_field_method_get("parent", |lua: &Lua, this: &PathObj| {
            let Some(parent) = this.0.parent() else { return Ok(LuaNil) };
            let parent: PathObj = PathObj(parent.to_owned());
            Ok(parent.into_lua(lua)?)
        });

        // The type of PathObj
        fields.add_field_method_get("type", |_, this: &PathObj| {
            if this.0.is_dir() {
                Ok("directory")
            } else if this.0.is_file() {
                Ok("file")
            } else if this.0.is_symlink() {
                Ok("symlink")
            } else {
                Ok("unknown")
            }
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Join another path to the end, creating a new PathObj
        methods.add_async_method("join", |_, this: LuaUserDataRef<PathObj>, other: PathBuf| async move {
            Ok(PathObj(this.0.join(other)))
        });

        // Make the PathObj absolute, returning a new PathObj
        methods.add_async_method("absolute", |_, this: LuaUserDataRef<PathObj>, ()| async move {
            Ok(PathObj(this.0.canonicalize().map_err(|source| Error::InvalidPath { path: this.0.clone(), source })?))
        })
    }
}
