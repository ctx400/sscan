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
use kameo::{actor::ActorRef, error::BoxError, mailbox::unbounded::UnboundedMailbox, Actor};
use mlua::prelude::*;
use userscript_apis::register_version_apis;

/// Manages the Lua virtual machine and userscript APIs.
///
/// One of the core strengths of sscan is its high degree of
/// extensibility via its Lua userscript subsystem. Through userscripts,
/// one can register custom scan engines and configure just about every
/// aspect of sscan.
///
/// This actor encapsulates the Lua virtual machine and provides APIs
/// for managing the userscript environment. Its primary job is to
/// register Lua APIs that userscripts can use to customize sscan, and
/// it also handles the execution of the userscripts themselves.
///
/// # Usage
///
/// The recommended way to use [`LuaVM`] is to first instantiate the
/// [`System`](crate::system::System) actor, and then request an
/// [`ActorRef`] to the virtual machine.
///
/// # Example
///
/// ```
/// # use sscan::{lua_vm::{LuaVM, messages::ExecuteChunk}, system::{System, messages::GetActorLuaVM}};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Start the system actor. System will automatically start and manage LuaVM.
/// let system = kameo::spawn(System::default());
///
/// // Get an ActorRef to LuaVM.
/// let lua_vm = system.ask(GetActorLuaVM).await?.unwrap();
///
/// // Execute some Lua code in the userscript environment.
/// lua_vm.tell(ExecuteChunk::using(r#"
///     print("Hello, World!")
/// "#)).await?;
/// # Ok(())
/// # }
/// ```
pub struct LuaVM(Lua);

impl Actor for LuaVM {
    type Mailbox = UnboundedMailbox<Self>;

    async fn on_start(&mut self, _: ActorRef<Self>) -> Result<(), BoxError> {
        register_version_apis(&self.0)?;
        Ok(())
    }

    fn name() -> &'static str {
        "lua_vm"
    }
}

impl Default for LuaVM {
    fn default() -> Self {
        Self(Lua::new())
    }
}

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
    #[deprecated(
        since = "0.8.0",
        note = "Initialization now happens during actor startup."
    )]
    pub fn init() -> LuaResult<Self> {
        let lua: Lua = Lua::new();
        register_version_apis(&lua)?;

        Ok(Self(lua))
    }
}
