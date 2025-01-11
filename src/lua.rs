//! Creates and manages the Lua virtual machine.
//!
//! This module is responsible for creating and managing the Lua vurtual
//! machine, which is used for userscripts, configuration, and custom
//! scan engines.
//!

// Scope Imports
use mlua::prelude::*;


/// Initializes Lua and adds APIs to the global scope.
pub fn init() -> LuaResult<Lua> {
    let lua: Lua = Lua::new();
    Ok(lua)
}
