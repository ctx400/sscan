//! Creates and manages the Lua virtual machine.
//!
//! This module is responsible for creating and managing the Lua vurtual
//! machine, which is used for userscripts, configuration, and custom
//! scan engines.
//!

// Modules
mod version_info;

// Scope Imports
use mlua::prelude::*;
use version_info::register_version_apis;

/// Initializes Lua and adds APIs to the global scope.
pub fn init() -> LuaResult<Lua> {
    let lua: Lua = Lua::new();
    register_version_apis(&lua)?;

    Ok(lua)
}
