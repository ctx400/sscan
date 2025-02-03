//! # Register custom Lua scan engines.
//!
//! The [`UserEngine`] API provides methods to userscripts to register
//! custom scan engines. Each scan engine should receive a byte string
//! payload, returning true or false on match or non-match, respectively.
//!
//! ## Userscript API
//!
//! This is a userscript API. The API's functionality is registered with
//! the Lua virtual machine, where userscripts can call into it.
//!
//! ### API Usage Examples
//!
//! **For full API documentation, launch sscan in interactive mode and
//!   enter `help 'user_engines'`.**
//!
//! Register a simple scan engine that always returns true.
//!
//! ```lua
//! function engine_alwaystrue(payload)
//!     return true
//! end
//!
//! user_engines:register("alwaystrue", engine_alwaystrue)
//! ```
//!
//! Register a simple scan engine that matches "Hello World"
//!
//! ```lua
//! function engine_match_helloworld(payload)
//!     if string.find(payload, "Hello World") ~= nil then
//!         return true
//!     else
//!         return false
//!     end
//! end
//!
//! user_engines:register("match_helloworld", engine_match_helloworld)
//! ```

use crate::{actors::user_engine::{error::Error, messages::{RegisterUserEngine, ScanBytes}, UserEngine}, userscript_api::{include::{LuaFunction, LuaString, LuaUserData, LuaUserDataMethods, LuaUserDataRef}, ApiObject}};
use kameo::actor::WeakActorRef;
use mlua::ExternalError;

/// # The Userscript Scan Engine API
///
/// The userscript scan engine API exposes a function
/// `user_engines:register(<name>, <function>)` to the Lua userscript
/// environment. Once one or more userscript scan engines are registered,
/// a scan can be launched using `user_engines:scan(<string>)`, where
/// `<string>` may also be a bytestring.
pub struct UserEngineApi {
    /// Weak ref to the user engine actor.
    engine_ref: WeakActorRef<UserEngine>,
}

impl UserEngineApi {
    /// Creates a new Userscript Scan Engine API with no engines loaded.
    #[must_use]
    pub fn new(engine_ref: WeakActorRef<UserEngine>) -> Self {
        Self { engine_ref }
    }
}

impl LuaUserData for UserEngineApi {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method("register", |_, this: LuaUserDataRef<UserEngineApi>, (name, spec): (String, LuaFunction)| async move {
            if let Some(user_engine) = this.engine_ref.upgrade() {
                user_engine.ask(RegisterUserEngine::using(name, spec)).await.map_err(mlua::ExternalError::into_lua_err)?;
                Ok(())
            } else {
                Err(Error::NoUserEngine.into_lua_err())
            }
        });

        methods.add_async_method("scan", |_, this: LuaUserDataRef<UserEngineApi>, content: LuaString| async move {
            if let Some(user_engine) = this.engine_ref.upgrade() {
                // Convert `content` into a byte vector
                let scan_request: ScanBytes = content.as_bytes().to_vec().into();

                // Call the userscript scan engine service
                let scan_results: Vec<String> = user_engine.ask(scan_request).await.map_err(mlua::ExternalError::into_lua_err)?;
                Ok(scan_results)
            } else {
                Err(Error::NoUserEngine.into_lua_err())
            }
        });
    }
}

impl ApiObject for UserEngineApi {
    fn name(&self) -> &'static str {
        "user_engines"
    }
}
