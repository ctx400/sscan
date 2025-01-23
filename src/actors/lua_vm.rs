//! # Provides the Lua Userscript Environment
//!
//! The [`LuaVM`] actor provides a Lua 5.4 virtual machine, which in
//! turn provides the userscript environment. sscan is both configured
//! and driven by userscripts, and userscripts can also define custom
//! scan engines.
//!

use kameo::{mailbox::unbounded::UnboundedMailbox, Actor};
use mlua::prelude::*;

/// # An actor which hosts a Lua VM and userscript environment.
///
/// This actor instantiates and hosts the Lua 5.4 virtual machine which
/// in turn provides the userscript environment. The userscript
/// environment provides APIs into the inner workings of sscan for
/// customization, configuration, and defining custom scan engines.
pub struct LuaVM {
    /// The inner Lua 5.4 Virtual Machine
    vm: Lua
}

/// # [`LuaVM`] is an actor.
///
/// This means that the virtual machine runs on its own thread and
/// communicates with other Rust components via message-passing. This
/// allows the virtual machine to run alongside other asynchronous
/// subsystems while maintaining owned mutable state without locks.
impl Actor for LuaVM {
    type Mailbox = UnboundedMailbox<Self>;
}

impl Default for LuaVM {
    /// Creates a new [`LuaVM`] instance to be passed into [`kameo::spawn()`].
    ///
    /// # Example
    ///
    /// ```
    /// # use sscan::actors::lua_vm::LuaVM;
    /// # use kameo::actor::ActorRef;
    /// #
    /// # #[tokio::test]
    /// # async fn should_spawn_vm() {
    /// // Create a new userscript environment!
    /// let vm: ActorRef<LuaVM> = kameo::spawn(LuaVM::default());
    /// # }
    /// ```
    fn default() -> Self {
        Self {
            vm: Lua::new(),
        }
    }
}
