//! # Provides the Scan Input Queue
//!
//! The [`Queue`] actor provides an input queue for files and raw data
//! items. Any data item implementing [`DataItem`] can be enqueued for
//! scanning.
//!
//! ## Interacting with the Input Queue
//!
//! [`Queue`] is an asynchronous actor, meaning it runs on its own
//! independent thread and has full control over its own mutable state.
//! Interaction with the queue is done through message passing.
//!
//! See the [`messages`] module to learn about the various types of
//! messages that can be sent to the queue to interact with it, along
//! with usage and code examples.
//!

pub mod data_item;
pub mod error;
pub mod messages;

use super::lua_vm::{messages::RegisterUserApi, LuaVM};
use crate::userscript_api::queue_api::QueueApi;
use data_item::DataItem;
use error::Error as QueueError;
use kameo::{
    actor::{ActorRef, WeakActorRef},
    error::BoxError,
    mailbox::unbounded::UnboundedMailbox,
    Actor,
};
use std::collections::VecDeque;

/// # The Global Scan Queue
///
/// This actor provides a global scan queue. Userscripts can enqueue
/// files and other items implementing trait [`DataItem`]. The queue
/// is used for efficiently sending input items to all scan engines.
pub struct Queue {
    /// A double-ended queue storing items implementing [`DataItem`]
    items: VecDeque<Box<dyn DataItem>>,

    /// Weak ref to the Lua virtual machine, for registering the API.
    lua_vm: WeakActorRef<LuaVM>,
}

/// # [`Queue`] is an actor.
///
/// This means that the queue runs on its own thread and communicates
/// with other Rust components via message passing. This allows the
/// queue to run alongside other asynchronous subsystems while
/// maintaining owned mutable state without locks.
impl Actor for Queue {
    type Mailbox = UnboundedMailbox<Self>;

    /// On startup, register the userscript API and help topics.
    async fn on_start(&mut self, queue: ActorRef<Self>) -> Result<(), BoxError> {
        if let Some(lua_vm) = self.lua_vm.upgrade() {
            let queue_api: QueueApi = QueueApi::new(queue.downgrade());
            lua_vm.ask(RegisterUserApi::with(queue_api)).await?;
            Ok(())
        } else {
            Err(Box::new(QueueError::NoLuaVm))
        }
    }
}

impl Queue {
    /// Create a global scan queue.
    ///
    /// Spawns a new [`Queue`] actor with a default queue
    /// size of `ZERO`. The queue will dynamically resize as needed.
    ///
    /// **Efficiency**: A queue of size zero will allocate very
    /// frequently when items are enqueued. It is recommended to use
    /// [`Queue::spawn_with_size()`] to choose a
    /// reasonable starting capacity.
    #[must_use]
    pub fn spawn(vm: WeakActorRef<LuaVM>) -> ActorRef<Self> {
        let actor: Queue = Self {
            items: VecDeque::new(),
            lua_vm: vm,
        };
        kameo::spawn(actor)
    }

    /// Create a global scan queue with given capacity.
    ///
    /// Spawns a new [`Queue`] actor with the provided starting capacity.
    /// This is recommended over [`Queue::spawn()`] as the initial
    /// capacity can be tuned to help avoid excessive allocations.
    #[must_use]
    pub fn spawn_with_size(vm: WeakActorRef<LuaVM>, capacity: usize) -> ActorRef<Self> {
        let actor: Queue = Self {
            items: VecDeque::with_capacity(capacity),
            lua_vm: vm,
        };
        kameo::spawn(actor)
    }
}
