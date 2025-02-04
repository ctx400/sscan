//! # Provides the Userscript Scan Engine Service
//!
//! The [`UserEngine`] actor provides a mechanism for userscripts to
//! register custom scan engines. Each custom scan engine is a valid Lua
//! function, which must accept a single argument of Lua type `string`,
//! and which must return a single argument `bool`.
//!
//! ## Interacting with the Userscript Scan Engine Service.
//!
//! [`UserEngine`] is an asynchronous actor, meaning it runs on its own
//! independent thread and has full control over its own mutable state.
//! Interaction with the queue is done throught message passing.
//!
//! See the [`messages`] module to learn about the various types of
//! messages that can be sent to the userscript scan engine service to
//! interact with it, along with usage and code examples.
//!

pub mod error;
pub mod messages;

use crate::{
    actors::lua_vm::{messages::RegisterUserApi, LuaVM},
    userscript_api::user_engine_api::UserEngineApi,
};
use kameo::{
    actor::{ActorRef, WeakActorRef},
    error::BoxError,
    mailbox::unbounded::UnboundedMailbox,
    Actor,
};
use mlua::Function;
use std::collections::HashMap;

/// # The Userscript Scan Engine Service
///
/// This actor provides a service for registering userscript-provided
/// scan engines, as well as invoking scans against all registered
/// engines for any byte vector.
pub struct UserEngine {
    /// Stores all registered userscript scan engines.
    engines: HashMap<String, Function>,

    /// Weak ref to the Lua virtual machine, for registering the API.
    lua_vm: WeakActorRef<LuaVM>,
}

impl Actor for UserEngine {
    type Mailbox = UnboundedMailbox<Self>;

    async fn on_start(&mut self, user_engine: ActorRef<Self>) -> Result<(), BoxError> {
        if let Some(lua_vm) = self.lua_vm.upgrade() {
            let user_eng_api: UserEngineApi = UserEngineApi::new(user_engine.downgrade());
            lua_vm.tell(RegisterUserApi::with(user_eng_api)).await?;
            Ok(())
        } else {
            Ok(())
        }
    }
}

impl UserEngine {
    /// Spawn a new [`UserEngine`] with zero initial capacity.
    ///
    /// NOTE: It is more favorable to use
    /// [`UserEngine::spawn_with_capacity()`], which allows setting an
    /// initial allocation size that makes sense. Otherwise, the
    /// standard `spawn()` function will allocate very often.
    #[must_use]
    pub fn spawn(vm: WeakActorRef<LuaVM>) -> ActorRef<Self> {
        let engine: Self = Self {
            engines: HashMap::new(),
            lua_vm: vm,
        };
        kameo::spawn(engine)
    }

    /// Spawn a new [`UserEngine`] with the given initial capacity.
    #[must_use]
    pub fn spawn_with_capacity(vm: WeakActorRef<LuaVM>, capacity: usize) -> ActorRef<Self> {
        let engine: Self = Self {
            engines: HashMap::with_capacity(capacity),
            lua_vm: vm,
        };
        kameo::spawn(engine)
    }
}
