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
//! For full API documentation, launch sscan in interactive mode and
//! enter `help 'queue'`, or see [`topics::queue`].
//!
//! [global scan queue]: crate::actors::queue::Queue
//! [`topics::queue`]: crate::userscript_api::help_system::topics::queue

use crate::{
    actors::queue::{
        data_item::{FileDatum, RawDatum},
        error::Error as QueueError,
        messages::{Dequeue, Enqueue, GetLength},
        Queue,
    },
    userscript_api::{
        fs_api::path_obj::PathObj,
        include::{LuaEither, LuaUserDataRef},
        ApiObject,
    },
};
use kameo::actor::WeakActorRef;
use mlua::{ExternalError, Lua, UserData, UserDataRef};
use std::path::PathBuf;

/// # Global Scan Queue Userscript API
///
/// This [`ApiObject`] is exposed to the Lua userscript environment,
/// allowing for scripts to queue and dequeue [`DataItem`] objects
/// programmatically.
///
/// ## API Docs
///
/// To see detailed help for this API, launch sscan and call
/// `help 'queue'`. Alternatively, the docs for this API are
/// available [here](super::help_system::topics::queue).
///
/// [`DataItem`]: crate::actors::queue::data_item::DataItem
pub struct QueueApi(WeakActorRef<Queue>);

impl QueueApi {
    /// Create the API object for [registration] with [`LuaVM`].
    ///
    /// [registration]: crate::actors::lua_vm::messages::RegisterUserApi
    /// [`LuaVM`]: crate::actors::lua_vm::LuaVM
    #[must_use]
    pub fn new(queue: WeakActorRef<Queue>) -> Self {
        Self(queue)
    }
}

impl UserData for QueueApi {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method("add_raw", queue_add_raw);
        methods.add_async_method("add_file", queue_add_file);
        methods.add_async_method("dequeue", queue_dequeue);
        methods.add_async_method("len", queue_len);
        methods.add_async_meta_method("__len", queue_len);
    }
}

impl ApiObject for QueueApi {
    fn name(&self) -> &'static str {
        "queue"
    }
}

/// Userscript function `queue:add_raw(name, data)`
async fn queue_add_raw(
    _: Lua,
    this: UserDataRef<QueueApi>,
    (name, content): (String, String),
) -> mlua::Result<()> {
    if let Some(queue) = this.0.upgrade() {
        let data_item: Box<RawDatum> = RawDatum::new(&name, content.as_bytes().to_vec());
        if queue.ask(Enqueue::item(data_item)).await.is_err() {
            Err(QueueError::SendError.into_lua_err())
        } else {
            Ok(())
        }
    } else {
        Err(QueueError::NoGlobalQueue.into_lua_err())
    }
}

/// Userscript function `queue:add_file(path)`
async fn queue_add_file(
    _: Lua,
    this: UserDataRef<QueueApi>,
    path: LuaEither<PathBuf, LuaUserDataRef<PathObj>>,
) -> mlua::Result<()> {
    if let Some(queue) = this.0.upgrade() {
        let path: PathBuf = match path {
            LuaEither::Left(pb) => pb,
            LuaEither::Right(po) => po.0.clone(),
        };
        let data_item: Box<FileDatum> = FileDatum::new(path);
        if queue.ask(Enqueue::item(data_item)).await.is_err() {
            Err(QueueError::SendError.into_lua_err())
        } else {
            Ok(())
        }
    } else {
        Err(QueueError::NoGlobalQueue.into_lua_err())
    }
}

/// Userscript function `queue:dequeue()`
async fn queue_dequeue(
    _: Lua,
    this: UserDataRef<QueueApi>,
    (): (),
) -> mlua::Result<(String, Option<PathBuf>, impl mlua::IntoLua)> {
    if let Some(queue) = this.0.upgrade() {
        match queue.ask(Dequeue).await {
            Ok((name, path, content)) => {
                let content = mlua::String::wrap(content);
                Ok((name, path, content))
            }
            Err(error) => Err(error.into_lua_err()),
        }
    } else {
        Err(QueueError::NoGlobalQueue.into_lua_err())
    }
}

/// Userscript function `queue:len()`
async fn queue_len(_: Lua, this: UserDataRef<QueueApi>, (): ()) -> mlua::Result<usize> {
    if let Some(queue) = this.0.upgrade() {
        queue
            .ask(GetLength)
            .await
            .map_err(mlua::ExternalError::into_lua_err)
    } else {
        Err(QueueError::NoGlobalQueue.into_lua_err())
    }
}
