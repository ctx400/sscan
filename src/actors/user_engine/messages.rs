//! # Messages Accepted by [`UserEngine`]
//!
//! As an asynchronous actor, the userscript scan engine service
//! communicates with other actors and rust components through message
//! passing. This module defines the various messages that the
//! userscript scan engine service accepts, their parameters, replies,
//! and code examples.
//!
//! See each message to learn more about interacting with the userscript
//! scan engine service and to register custom scan engines.
//!

use kameo::message::{Context, Message};
use crate::{actors::user_engine::{UserEngine, error::{Error, UserEngineResult}}, userscript_api::include::{LuaFunction, LuaString}};

/// # Register a Userscript Scan Engine
///
/// A request for the [`UserEngine`] to register a custom userscript
/// scan engine for use during scans. Once registered, the custom scan
/// engine will be called on every request to [`ScanBytes`].
///
/// ## Reply
///
/// Expect no reply from the userscript scan engine service.
///
/// ## Example
///
/// For more help, see [`topics::user_engines`].
///
/// ```lua
/// user_engines:register('match_hello', function(p) return (p:find('hello') ~= nil) end)
/// ```
///
/// [`topics::user_engines`]: crate::userscript_api::help_system::topics::user_engines
pub struct RegisterUserEngine {
    /// Name to associate with the userscript scan engine
    name: String,

    /// The function to register as the userscript scan engine
    spec: LuaFunction,
}

impl Message<RegisterUserEngine> for UserEngine {
    type Reply = ();

    async fn handle(&mut self, msg: RegisterUserEngine, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        self.engines.insert(msg.name, msg.spec);
    }
}

impl RegisterUserEngine {
    /// Create a new [`RegisterUserEngine`] message.
    #[must_use]
    pub fn using(name: String, spec: LuaFunction) -> Self {
        Self { name, spec }
    }
}

/// # Scan a byte vector against all registered userscript engines.
///
/// A request for [`UserEngine`] to scan a [`Vec<u8>`] against all
/// registered userscript scan engines. The userscript scan engine
/// service will pass the byte vector to each engine individually,
/// recording the name of each engine that returned [`true`](bool).
///
/// ## Reply
///
/// Expect a reply of type [`UserEngineResult<Vec<String>>`], where each
/// [`String`] in the vector is the name of a scan engine that returned
/// a match result of [`true`](bool).
///
/// ## Example
///
/// For more help, see [`topics::user_engines`].
///
/// ```lua
/// local results = user_engines:scan('blablabla-some dummy data-\x41\x42\x43\x44\x45')
/// ```
///
/// [`topics::user_engines`]: crate::userscript_api::help_system::topics::user_engines
pub struct ScanBytes(Vec<u8>);

impl Message<ScanBytes> for UserEngine {
    type Reply = UserEngineResult<Vec<String>>;

    async fn handle(&mut self, msg: ScanBytes, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        // The `_vm_guard` keeps LuaVM alive long enough to call all Lua scan engines.
        if let Some(_vm_guard) = self.lua_vm.upgrade() {
            // Stores a list of matching engines for `msg`
            let mut results: Vec<String> = Vec::with_capacity(1024);

            // Invoke each scan engine and get its result.
            for (name, spec) in &self.engines {
                // Convert the `Vec<u8>` into a Lua bytestring
                let bytestring = LuaString::wrap(msg.0.as_slice());

                // Invoke the scan engine and get the result.
                let result: UserEngineResult<bool> = spec.call_async(bytestring).await.map_err(|err: mlua::Error| Error::engine_invocation(name.clone(), err));
                if result? {
                    results.push(name.clone());
                }
            }
            Ok(results)
        } else {
            Err(Error::NoLuaVm)
        }
    }
}

impl From<Vec<u8>> for ScanBytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}
