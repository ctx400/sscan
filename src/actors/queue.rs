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

pub mod error;
pub mod data_item;
pub mod messages;

use std::collections::VecDeque;
use data_item::DataItem;
use kameo::{mailbox::unbounded::UnboundedMailbox, Actor};

pub struct Queue {
    items: VecDeque<Box<dyn DataItem>>,
}

impl Actor for Queue {
    type Mailbox = UnboundedMailbox<Self>;
}

impl Queue {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: VecDeque::with_capacity(capacity),
        }
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::with_capacity(2048)
    }
}
