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

use serde::Serialize;

use crate::userscript_api::{
    fs_api::error::Error,
    include::{
        IntoLua, Lua, LuaEither, LuaNil, LuaUserData, LuaUserDataFields, LuaUserDataMethods,
        LuaUserDataRef, LuaValue,
    },
};
use std::{path::PathBuf, time::UNIX_EPOCH};

/// Represents a Directory Entry
#[derive(Serialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathObj(pub PathBuf);

impl LuaUserData for PathObj {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        // The PathObj's full path.
        fields.add_field_method_get("path", |_, this: &PathObj| Ok(this.0.clone()));

        // Filename of the PathObj.
        fields.add_field_method_get("name", |lua: &Lua, this: &PathObj| {
            let Some(filename) = this.0.file_name() else {
                return Ok(LuaNil);
            };
            let filename: LuaValue = filename.into_lua(lua)?;
            if filename.is_string() {
                Ok(filename)
            } else {
                Ok(LuaNil)
            }
        });

        // File extension of the PathObj.
        fields.add_field_method_get("ext", |lua: &Lua, this: &PathObj| {
            let Some(extension) = this.0.extension() else {
                return Ok(LuaNil);
            };
            let extension: LuaValue = extension.into_lua(lua)?;
            if extension.is_string() {
                Ok(extension)
            } else {
                Ok(LuaNil)
            }
        });

        // The file stem (filename without extension) of the PathObj.
        fields.add_field_method_get("stem", |lua: &Lua, this: &PathObj| {
            let Some(stem) = this.0.file_stem() else {
                return Ok(LuaNil);
            };
            let stem: LuaValue = stem.into_lua(lua)?;
            if stem.is_string() {
                Ok(stem)
            } else {
                Ok(LuaNil)
            }
        });

        // The parent PathObj
        fields.add_field_method_get("parent", |lua: &Lua, this: &PathObj| {
            let Some(parent) = this.0.parent() else {
                return Ok(LuaNil);
            };
            let parent: PathObj = PathObj(parent.to_owned());
            parent.into_lua(lua)
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

        fields.add_field_method_get("size", |_, this: &PathObj| {
            let Ok(metadata) = this.0.metadata() else {
                return Ok(LuaNil);
            };

            // Lua integers are always i64. Cannot get around this, so
            // we must accept the possibility of wrapping.
            #[allow(clippy::cast_possible_wrap)]
            Ok(LuaValue::Integer(metadata.len() as i64))
        });

        fields.add_field_method_get("atime", |_, this: &PathObj| {
            let Ok(metadata) = this.0.metadata() else {
                return Ok(LuaNil);
            };
            let Ok(atime) = metadata.accessed() else {
                return Ok(LuaNil);
            };
            let Ok(atime) = atime.duration_since(UNIX_EPOCH) else {
                return Ok(LuaNil);
            };

            // Lua integers are always i64. Cannot get around this, so
            // we must accept the possibility of wrapping.
            #[allow(clippy::cast_possible_wrap)]
            Ok(LuaValue::Integer(atime.as_secs() as i64))
        });

        fields.add_field_method_get("mtime", |_, this: &PathObj| {
            let Ok(metadata) = this.0.metadata() else {
                return Ok(LuaNil);
            };
            let Ok(mtime) = metadata.modified() else {
                return Ok(LuaNil);
            };
            let Ok(mtime) = mtime.duration_since(UNIX_EPOCH) else {
                return Ok(LuaNil);
            };

            // Lua integers are always i64. Cannot get around this, so
            // we must accept the possibility of wrapping.
            #[allow(clippy::cast_possible_wrap)]
            Ok(LuaValue::Integer(mtime.as_secs() as i64))
        });

        fields.add_field_method_get("ctime", |_, this: &PathObj| {
            let Ok(metadata) = this.0.metadata() else {
                return Ok(LuaNil);
            };
            let Ok(ctime) = metadata.created() else {
                return Ok(LuaNil);
            };
            let Ok(ctime) = ctime.duration_since(UNIX_EPOCH) else {
                return Ok(LuaNil);
            };

            // Lua integers are always i64. Cannot get around this, so
            // we must accept the possibility of wrapping.
            #[allow(clippy::cast_possible_wrap)]
            Ok(LuaValue::Integer(ctime.as_secs() as i64))
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Join another path to the end, creating a new PathObj
        methods.add_async_method(
            "join",
            |_,
             this: LuaUserDataRef<PathObj>,
             other: LuaEither<PathBuf, LuaUserDataRef<PathObj>>| async move {
                let other: PathBuf = match other {
                    LuaEither::Left(pb) => pb,
                    LuaEither::Right(po) => po.0.clone(),
                };
                Ok(PathObj(this.0.join(other)))
            },
        );

        // Make the PathObj absolute, returning a new PathObj
        methods.add_async_method(
            "absolute",
            |_, this: LuaUserDataRef<PathObj>, ()| async move {
                Ok(PathObj(this.0.canonicalize().map_err(|source| {
                    Error::InvalidPath {
                        path: this.0.clone(),
                        source,
                    }
                })?))
            },
        );

        // Same as PathObj:join, but uses concat syntax
        methods.add_async_meta_method(
            "__concat",
            |_,
             this: LuaUserDataRef<PathObj>,
             other: LuaEither<PathBuf, LuaUserDataRef<PathObj>>| async move {
                let other: PathBuf = match other {
                    LuaEither::Left(pb) => pb,
                    LuaEither::Right(po) => po.0.clone(),
                };
                Ok(PathObj(this.0.join(other)))
            },
        );

        // Equivalent to PathObj.parent, but using the unary `-` syntax.
        methods.add_async_meta_method(
            "__unm",
            |lua: Lua, this: LuaUserDataRef<PathObj>, ()| async move {
                let Some(parent) = this.0.parent() else {
                    return Ok(LuaNil);
                };
                let parent: PathObj = PathObj(parent.to_owned());
                parent.into_lua(&lua)
            },
        );

        // Returns true if two path objects are equal
        methods.add_async_meta_method(
            "__eq",
            |_, this: LuaUserDataRef<PathObj>, other: LuaUserDataRef<PathObj>| async move {
                Ok(*this == *other)
            },
        );

        // Returns true if path A is lexicographically before path B.
        methods.add_async_meta_method(
            "__lt",
            |_, this: LuaUserDataRef<PathObj>, other: LuaUserDataRef<PathObj>| async move {
                Ok(*this < *other)
            },
        );

        // Returns true if A is lexicographically before or equal to B.
        methods.add_async_meta_method(
            "__le",
            |_, this: LuaUserDataRef<PathObj>, other: LuaUserDataRef<PathObj>| async move {
                Ok(*this <= *other)
            },
        );

        // Converts the PathObj to a raw string path
        methods.add_async_meta_method(
            "__tostring",
            |_, this: LuaUserDataRef<PathObj>, ()| async move { Ok(this.0.clone()) },
        );
    }
}
