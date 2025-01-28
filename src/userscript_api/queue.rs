//! # Add or remove items from the global scan queue
//!
//! The [`QueueApi`] allows userscripts to enqueue data items with the
//! [global scan queue]. As such, it is possible to programmatically
//! enqueue files and other data, rather than just providing a static
//! file list.
//!
//! ## Userscript API
//!
//! This is a userscript API. The API's functionality is registered with
//! the Lua virtual machine, where userscripts can call into it.
//!
//! ## API Usage Examples
//!
//! **For full API documentation, launch sscan in interactive mode and
//!   enter `help 'queue'`.**
//!
//! ```lua
//! Usage: queue:add_file 'path/to/file'
//!     Enqueue a file for scanning.
//!
//! Usage: queue:add_raw(name, data)
//!     Enqueue a raw Lua string/bytestring for scanning.
//!
//! Usage: queue:dequeue()
//!     Pull the data item at the front of the queue.
//! ```
//!
//! [global scan queue]: crate::actors::queue::Queue

use std::path::PathBuf;
use kameo::actor::WeakActorRef;
use mlua::{ExternalError, Lua, UserData, UserDataRef};
use crate::actors::queue::{data_item::{FileDatum, RawDatum}, error::Error as QueueError, messages::{Dequeue, Enqueue}, Queue};
use super::ApiObject;

pub struct QueueApi(WeakActorRef<Queue>);

impl QueueApi {
    pub fn new(queue: WeakActorRef<Queue>) -> Self {
        Self(queue)
    }
}

impl UserData for QueueApi {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method("add_raw", queue_add_raw);
        methods.add_async_method("add_file", queue_add_file);
        methods.add_async_method("dequeue", queue_dequeue);
    }
}

impl ApiObject for QueueApi {
    fn name(&self) -> &'static str {
        "queue"
    }
}

/// Userscript function `queue:add_raw(name, data)`
async fn queue_add_raw(_: Lua, this: UserDataRef<QueueApi>, (name, content): (String, String)) -> mlua::Result<()> {
    if let Some(queue) = this.0.upgrade() {
        let data_item: Box<RawDatum> = RawDatum::new(&name, content.as_bytes().to_vec());
        if let Err(_) = queue.ask(Enqueue::item(data_item)).await {
            Err(QueueError::SendError.into_lua_err())
        } else {
            Ok(())
        }
    } else {
        Err(QueueError::NoGlobalQueue.into_lua_err())
    }
}

/// Userscript function `queue:add_file(path)`
async fn queue_add_file(_: Lua, this: UserDataRef<QueueApi>, path: PathBuf) -> mlua::Result<()> {
    if let Some(queue) = this.0.upgrade() {
        let data_item: Box<FileDatum> = FileDatum::new(path);
        if let Err(_) = queue.ask(Enqueue::item(data_item)).await {
            Err(QueueError::SendError.into_lua_err())
        } else {
            Ok(())
        }
    } else {
        Err(QueueError::NoGlobalQueue.into_lua_err())
    }
}

/// Userscript function `queue:dequeue()`
async fn queue_dequeue(_: Lua, this: UserDataRef<QueueApi>, _: ()) -> mlua::Result<(String, Option<PathBuf>, Vec<u8>)> {
    if let Some(queue) = this.0.upgrade() {
        match queue.ask(Dequeue).await {
            Ok(dqi) => Ok(dqi),
            Err(error) => Err(error.into_lua_err()),
        }
    } else {
        Err(QueueError::NoGlobalQueue.into_lua_err())
    }
}
