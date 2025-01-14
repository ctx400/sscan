//! # Lua Virtual Machine and Userscript Environment
//!
//! The [`LuaVM`] actor provides a userscript environment for extensive
//! customization of sscan. Users can modify behavior, change scan
//! engine configuration, or even register custom scan engines.
//!
//! ## API Documentation
//!
//! For information about the various APIs sscan exposes to userscripts,
//! see each of the various modules under [`userscript_apis`]
//!

// Modules
pub mod messages;
pub mod userscript_apis;

// Scope Imports
use kameo::Actor;
use mlua::prelude::*;
use userscript_apis::register_version_apis;

/// Manages the Lua virtual machine and userscript APIs.
///
/// One of the core strengths of sscan is its high degree of
/// extensibility via its Lua userscript subsystem. Through userscripts,
/// one can register custom scan engines and configure just about every
/// aspect of sscan.
///
/// This struct encapsulates the Lua virtual machine and provides APIs
/// for managing the userscript environment. Its primary job is to
/// register Lua APIs that userscripts can use to customize sscan, and
/// it also handles the execution of the userscripts themselves.
///
/// To set up a new userscript environment, see [`LuaVM::init()`].
///
#[derive(Actor)]
pub struct LuaVM(Lua);

impl LuaVM {
    /// Initializes Lua and adds core APIs to the global scope.
    ///
    /// This function creates a new Lua virtual machine and registers
    /// the core set of userscript APIs. This function is meant to be
    /// called during the initialization of a [`LuaVM`] actor.
    ///
    /// # Errors
    ///
    /// Any errors returning from this function are Lua errors. If a Lua
    /// error occurs, this is probably a bug and should be reported.
    ///
    /// # Example
    ///
    /// ```
    /// # use mlua::prelude::LuaResult;
    /// # use sscan::lua_vm::{LuaVM, messages::ExecuteChunk};
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// // Create and spawn a LuaVM actor.
    /// let vm = kameo::spawn(LuaVM::init()?);
    ///
    /// // Call the version() function in the virtual machine.
    /// let exec_request = ExecuteChunk::using("version()");
    /// vm.ask(exec_request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn init() -> LuaResult<Self> {
        let lua: Lua = Lua::new();
        register_version_apis(&lua)?;

        Ok(Self(lua))
    }
}
