//! # Provides a high-level scanning interface
//!
//! The [`ScanMgr`] actor provides a high-level scanning operations
//! interface which automates a lot of the tasks userscripts perform at
//! a lower level, such as pulling from the scan queue and distributing
//! [`DataItem`] objects to each scan engine.
//!
//! ## Interacting with the Scan Manager Service
//!
//! [`ScanMgr`] is an asynchronous actor, meaning it runs on its own
//! independent thread and has full control over its own mutable state.
//! Interaction with the scan manager service is done through message
//! passing.
//!
//! See the [`messages`] module to learn about the various types of
//! messages that can be sent to the scan manager service to interact
//! with it, along with usage and code examples.
//!
//! [`DataItem`]: crate::actors::queue::data_item::DataItem

pub mod error;
pub mod messages;
pub mod reply;

use crate::{
    actors::{
        lua_vm::{messages::RegisterUserApi, LuaVM},
        queue::Queue,
        scanmgr::error::Error,
        user_engine::UserEngine,
    },
    userscript_api::scanmgr_api::ScanMgrApi,
};
use kameo::{
    actor::{ActorRef, WeakActorRef},
    error::BoxError,
    mailbox::unbounded::UnboundedMailbox,
    Actor,
};

/// # The Scan Manager Service
///
/// This actor provides a high-level interface to initiate scans against
/// all activated scan engines without manually dealing with dequeueing
/// data items or managing each engine's results.
pub struct ScanMgr {
    /// Weak ref to [`LuaVM`], for registering the API.
    lua_ref: WeakActorRef<LuaVM>,

    /// Weak ref to the [`Queue`], for dequeueing [`DataItem`] objects.
    ///
    /// [`DataItem`]: crate::actors::queue::data_item::DataItem
    queue_ref: WeakActorRef<Queue>,

    /// Weak ref to the [`UserEngine`], for calling userscript engines.
    user_engine_ref: WeakActorRef<UserEngine>,
}

impl Actor for ScanMgr {
    type Mailbox = UnboundedMailbox<Self>;

    async fn on_start(&mut self, actor: ActorRef<Self>) -> Result<(), BoxError> {
        // Get a strongref to LuaVM or fail
        let Some(lua_vm) = self.lua_ref.upgrade() else {
            return Err(Box::new(Error::NoLuaVm));
        };

        // Register the userscript API
        let scanmgr_api: ScanMgrApi = ScanMgrApi::new(actor.downgrade());
        lua_vm.tell(RegisterUserApi::with(scanmgr_api)).await?;
        Ok(())
    }
}

impl ScanMgr {
    /// Spawn a new [`ScanMgr`]
    #[must_use]
    pub fn spawn(
        vm: WeakActorRef<LuaVM>,
        queue: WeakActorRef<Queue>,
        user_engine: WeakActorRef<UserEngine>,
    ) -> ActorRef<Self> {
        let actor: Self = Self {
            lua_ref: vm,
            queue_ref: queue,
            user_engine_ref: user_engine,
        };
        kameo::spawn(actor)
    }
}
