//! # Provides the Lua Userscript Environment
//!
//! The [`LuaVM`] actor provides a Lua 5.4 virtual machine, which in
//! turn provides the userscript environment. sscan is both configured
//! and driven by userscripts, and userscripts can also define custom
//! scan engines.
//!
//! ## Interacting with the Virtual Machine
//!
//! [`LuaVM`] is an asynchronous actor, meaning it runs the Lua virtual
//! machine on its own thread independently and has full control over
//! its own state. Interaction with the VM is done by message-passing.
//!
//! See the [`messages`] module to learn about the various types of
//! messages that can be sent to the VM to interact with it, along with
//! usage and code examples.
//!

pub mod error;
pub mod messages;

use crate::{
    actors::queue::Queue,
    userscript_api::{about_api::AboutApi, help_system::HelpSystem},
};
use kameo::{actor::ActorRef, error::BoxError, mailbox::unbounded::UnboundedMailbox, Actor};
use messages::RegisterUserApi;
use mlua::prelude::*;

use super::user_engine::UserEngine;

/// # An actor which hosts a Lua VM and userscript environment.
///
/// This actor instantiates and hosts the Lua 5.4 virtual machine which
/// in turn provides the userscript environment. The userscript
/// environment provides APIs into the inner workings of sscan for
/// customization, configuration, and defining custom scan engines.
pub struct LuaVM {
    /// The inner Lua 5.4 Virtual Machine
    vm: Lua,

    /// Reference to the [`Queue`] service
    queue: Option<ActorRef<Queue>>,

    /// Reference to the [`UserEngine`] service
    user_engine: Option<ActorRef<UserEngine>>,
}

/// # [`LuaVM`] is an actor.
///
/// This means that the virtual machine runs on its own thread and
/// communicates with other Rust components via message-passing. This
/// allows the virtual machine to run alongside other asynchronous
/// subsystems while maintaining owned mutable state without locks.
impl Actor for LuaVM {
    type Mailbox = UnboundedMailbox<Self>;

    async fn on_start(&mut self, lua_vm: ActorRef<Self>) -> Result<(), BoxError> {
        // Spawn other actors
        let queue: ActorRef<Queue> = Queue::spawn_with_size(lua_vm.downgrade(), 16384);
        let user_engine: ActorRef<UserEngine> = UserEngine::spawn_with_capacity(lua_vm.downgrade(), 128);

        // Register auxillary userscript APIs
        lua_vm
            .tell(RegisterUserApi::with(HelpSystem::default()))
            .await?;
        lua_vm
            .tell(RegisterUserApi::with(AboutApi::default()))
            .await?;

        // Link all actors to self
        lua_vm.link(&queue).await;
        lua_vm.link(&user_engine).await;

        // Store references to the other actors
        self.queue = Some(queue);
        self.user_engine = Some(user_engine);
        Ok(())
    }
}

impl LuaVM {
    /// Spawn a new Lua virtual machine in default execution mode.
    #[must_use]
    pub fn spawn() -> ActorRef<Self> {
        let lua_vm: Self = Self {
            vm: Lua::new(),
            queue: None,
            user_engine: None,
        };
        kameo::spawn(lua_vm)
    }

    /// Spawn a new Lua virtual machine with unsafe libraries loaded.
    ///
    /// This function spawns [`LuaVM`] in unsafe mode. This means unsafe
    /// libraries, such as `debug`, are loaded into Lua. *Be careful
    /// with this!*
    ///
    /// ## Safety
    ///
    /// Incorrect use of the Lua `debug` library or other unsafe
    /// libraries can cause *undefined behavior*, leading to panics or
    /// unpredictable side effects! Unsafe mode is only intended for
    /// advanced users, and testing purposes only. Production
    /// userscripts should *never* rely on any functionality provided by
    /// unsafe mode.
    #[must_use]
    pub unsafe fn spawn_unsafe() -> ActorRef<Self> {
        let lua_vm: Self = Self {
            vm: Lua::unsafe_new(),
            queue: None,
            user_engine: None,
        };
        kameo::spawn(lua_vm)
    }
}
