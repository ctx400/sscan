//! # APIs for Lua userscripts
//!
//! This module is responsible for creating and managing the Lua vurtual
//! machine, which is used for userscripts, configuration, and custom
//! scan engines.
//!
//! ## API Documentation
//!
//! For information about the various APIs sscan exposes to userscripts,
//! see each of the various modules under this one, for example,
//! [version_info].
//!

// Modules
pub mod version_info;

// Scope Imports
use mlua::prelude::*;
use version_info::register_version_apis;

pub struct LuaVM(Lua);

impl LuaVM {
    /// Initializes Lua and adds core APIs to the global scope.
    pub fn init() -> LuaResult<Self> {
        let lua: Lua = Lua::new();
        register_version_apis(&lua)?;

        Ok(Self(lua))
    }

    /// Execute a Lua code snippet in the virtual machine.
    pub fn exec(&self, script: &str) -> LuaResult<()> {
        self.0.load(script).exec()
    }
}
