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

pub struct Enqueue(Box<dyn DataItem>);

impl Message<Enqueue> for Queue {
    type Reply = ();

    async fn handle(&mut self, msg: Enqueue, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        self.items.push_back(msg.0);
    }
}

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
