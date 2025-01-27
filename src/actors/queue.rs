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
use kameo::{mailbox::unbounded::UnboundedMailbox, Actor};
use std::collections::VecDeque;

/// # The Global Scan Queue
///
/// This actor provides a global scan queue. Userscripts can enqueue
/// files and other items implementing trait [`DataItem`]. The queue
/// is used for efficiently sending input items to all scan engines.
pub struct Queue {
    /// A double-ended queue storing items implementing [`DataItem`]
    items: VecDeque<Box<dyn DataItem>>,
}

/// # [`Queue`] is an actor.
///
/// This means that the queue runs on its own thread and communicates
/// with other Rust components via message passing. This allows the
/// queue to run alongside other asynchronous subsystems while
/// maintaining owned mutable state without locks.
impl Actor for Queue {
    type Mailbox = UnboundedMailbox<Self>;
}

impl Queue {
    /// Create a new [`Queue`] of capacity `ZERO`.
    ///
    /// **Efficiency**: A queue of size zero will allocate very
    /// frequently when items are enqueued. It is recommended to use
    /// [`Queue::with_capacity()`] or [`Queue::default()`] to choose a
    /// reasonable starting capacity.
    #[must_use]
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    /// Create a new [`Queue`] of capacity [`usize`] data items.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: VecDeque::with_capacity(capacity),
        }
    }
}

impl Default for Queue {
    /// Create a new [`Queue`] of capacity `2048` data items.
    fn default() -> Self {
        Self::with_capacity(2048)
    }
}
