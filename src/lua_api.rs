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
//! [`version_info`].
//!

// Modules
pub mod version_info;

// Scope Imports
use mlua::prelude::*;
use version_info::register_version_apis;

pub struct LuaVM(Lua);

impl LuaVM {
    /// Initializes Lua and adds core APIs to the global scope.
    ///
    /// This function creates a new Lua virtual machine and registers
    /// the core set of userscript APIs.
    ///
    /// ## Errors
    ///
    /// Any errors returning from this function are Lua errors. If a Lua
    /// error occurs, this is probably a bug and should be reported.
    ///
    pub fn init() -> LuaResult<Self> {
        let lua: Lua = Lua::new();
        register_version_apis(&lua)?;

        Ok(Self(lua))
    }

    /// Execute a Lua code snippet in the virtual machine.
    ///
    /// This function takes a snippet of Lua code and executes it in the
    /// virtual machine. It provides the core functionality of loading
    /// and running userscripts.
    ///
    /// ## Errors
    ///
    /// Errors returned from this function are Lua errors. Most likely,
    /// the Lua code passed to the `script` argument had a syntax error
    /// or otherwise contained logic errors. If such an error is
    /// returned, check the Lua snippet for errors and try again.
    ///
    pub fn exec(&self, script: &str) -> LuaResult<()> {
        self.0.load(script).exec()
    }
}
