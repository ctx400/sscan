//! # Messages Accepted by [`Queue`]
//!
//! As an asynchronous actor, the item queue communicates with other
//! actors and rust components through message passing. This module
//! defines the various messages that the item queue accepts, their
//! parameters, replies, and code examples.
//!
//! See each message to learn more about interacting with the item
//! queue and to queue/dequeue data items.
//!

use super::{
    data_item::DataItem,
    error::{Error, QueueResult},
    Queue,
};
use kameo::message::{Context, Message};
use std::path::PathBuf;

/// # Push a [`DataItem`] into the scan queue.
///
/// A request for [`Queue`] to add a [`DataItem`] to the back of the
/// scan queue. Once enqueued, a data item can be later dequeued and
/// passed to any number of scan engines.
///
/// ## Reply
///
/// Expect no reply from the scan queue.
///
/// ## Example
///
/// ```
/// # use sscan::actors::{lua_vm::LuaVM, queue::{Queue, messages::Enqueue, data_item::RawDatum}};
/// # use kameo::actor::ActorRef;
/// # #[tokio::main]
/// # async fn main() {
/// // Create a new scan queue.
/// let lua_ref = kameo::spawn(LuaVM::default());
/// let queue = Queue::spawn(lua_ref.downgrade());
///
/// // Create a new data item for scanning
/// let data = RawDatum::new("hello_world", "blablabla-Hello World-blablabla");
///
/// // Enqueue the data item
/// queue.ask(Enqueue::item(data)).await.unwrap();
/// # }
/// ```
pub struct Enqueue(Box<dyn DataItem>);

impl Message<Enqueue> for Queue {
    type Reply = ();

    async fn handle(&mut self, msg: Enqueue, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        self.items.push_back(msg.0);
    }
}

impl Enqueue {
    /// Create a new enqueue request from a [`DataItem`].
    #[must_use]
    pub fn item(item: Box<dyn DataItem>) -> Self {
        Self(item)
    }
}

/// # Pop and realize a [`DataItem`] from the scan queue.
///
/// A request for [`Queue`] to pull a [`DataItem`] from the front of the
/// scan queue. Once pulled, the data item is [`realized`] before
/// returning to the sender.
///
/// ## Reply
///
/// Expect a reply of type [`QueueResult`].
///
/// ## Example
///
/// ```
/// # use sscan::actors::{lua_vm::LuaVM, queue::{Queue, messages::{Enqueue, Dequeue}, data_item::RawDatum}};
/// # use kameo::actor::ActorRef;
/// # #[tokio::main]
/// # async fn main() {
/// // Create a new scan queue.
/// let lua_ref = kameo::spawn(LuaVM::default());
/// let queue = Queue::spawn(lua_ref.downgrade());
///
/// // Create a new data item for scanning
/// let data = RawDatum::new("hello_world", "blablabla-Hello World-blablabla");
///
/// // Enqueue the data item
/// queue.ask(Enqueue::item(data)).await.unwrap();
///
/// // Now, dequeue the data item.
/// let (name, path, content) = queue.ask(Dequeue).await.unwrap();
/// assert_eq!(name, "hello_world");
/// assert_eq!(path, None);
/// assert_eq!(content, b"blablabla-Hello World-blablabla".to_vec());
/// # }
/// ```
/// [`realized`]: super::data_item::DataItem::realize()
pub struct Dequeue;

impl Message<Dequeue> for Queue {
    type Reply = QueueResult<(String, Option<PathBuf>, Vec<u8>)>;

    async fn handle(&mut self, _: Dequeue, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        if let Some(item) = self.items.pop_front() {
            Ok(item.realize()?)
        } else {
            Err(Error::empty())
        }
    }
}
