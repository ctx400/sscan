//! # Provides the Scan Input Queue
//!
//! The [`Queue`] actor provides an input queue for files and raw data
//! items. Any data item implementing [`DataItem`] can be enqueued for
//! scanning.
//!
//! ## Interacting with the Input Queue
//!
//! [`Queue`] is an asynchronous actor, meaning it runs on its own
//! independent thread and as full control over its own mutable state.
//! Interaction with the queue is done through message passing.
//!
//! See the [`messages`] module to learn about the various types of
//! messages that can be sent to the queue to interact with it, along
//! with usage and code examples.
//!

pub mod data_item;
pub mod error;
pub mod messages;

use data_item::DataItem;
use kameo::{actor::{ActorRef, WeakActorRef}, error::BoxError, mailbox::unbounded::UnboundedMailbox, Actor};
use std::collections::VecDeque;
use crate::userscript_api::queue::QueueApi;
use super::lua_vm::{messages::RegisterUserApi, LuaVM};
use error::Error as QueueError;

/// # The Global Scan Queue
///
/// This actor provides a global scan queue. Userscripts can enqueue
/// files and other items implementing trait [`DataItem`]. The queue
/// is used for efficiently sending input items to all scan engines.
pub struct Queue {
    /// A double-ended queue storing items implementing [`DataItem`]
    items: VecDeque<Box<dyn DataItem>>,

    /// WeakRef to the Lua virtual machine, for registering the API.
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
    /// Create a new [`Queue`] of capacity `ZERO`.
    ///
    /// **Efficiency**: A queue of size zero will allocate very
    /// frequently when items are enqueued. It is recommended to use
    /// [`Queue::with_capacity()`] to choose a
    /// reasonable starting capacity.
    #[must_use]
    pub fn new(vm: WeakActorRef<LuaVM>) -> Self {
        Self {
            items: VecDeque::new(),
            lua_vm: vm,
        }
    }

    /// Create a new [`Queue`] of capacity [`usize`] data items.
    #[must_use]
    pub fn with_capacity(vm: WeakActorRef<LuaVM>, capacity: usize) -> Self {
        Self {
            items: VecDeque::with_capacity(capacity),
            lua_vm: vm,
        }
    }
}
