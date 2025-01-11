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
pub struct LuaVM(Lua);

impl LuaVM {
    /// Initializes Lua and adds core APIs to the global scope.
    ///
    /// This function creates a new Lua virtual machine and registers
    /// the core set of userscript APIs.
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
    /// # use sscan::lua_api::LuaVM;
    /// # fn main() -> LuaResult<()> {
    /// // Create a Lua VM and print sscan version info.
    /// let vm: LuaVM = LuaVM::init()?;
    /// vm.exec("version()")?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// # Errors
    ///
    /// Errors returned from this function are Lua errors. Most likely,
    /// the Lua code passed to the `script` argument had a syntax error
    /// or otherwise contained logic errors. If such an error is
    /// returned, check the Lua snippet for errors and try again.
    ///
    /// # Example
    ///
    /// ```
    /// # use mlua::prelude::LuaResult;
    /// # use sscan::lua_api::LuaVM;
    /// # fn main() -> LuaResult<()> {
    /// // Create a VM and load a function.
    /// let vm: LuaVM = LuaVM::init()?;
    /// vm.exec(r#"
    ///     function say_hello()
    ///         print("Hello World!")
    ///     end
    /// "#)?;
    ///
    /// // Use this function in later userscripts
    /// vm.exec("say_hello()")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn exec(&self, script: &str) -> LuaResult<()> {
        self.0.load(script).exec()
    }
}
