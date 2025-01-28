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

use super::ApiObject;
use mlua::{Function, UserData};
use std::collections::HashMap;

/// # The Userscript Scan Engine API
///
/// The userscript scan engine API exposes a function
/// `user_engines:register(<name>, <function>)` to the Lua userscript
/// environment. Once one or more userscript scan engines are registered,
/// a scan can be launched using `user_engines:scan(<string>)`, where
/// `<string>` may also be a bytestring.
pub struct UserEngine {
    /// Holds the list of registered userscript scan engines.
    engines: HashMap<String, Function>,
}

impl UserEngine {
    /// Creates a new Userscript Scan Engine API with no engines loaded.
    #[must_use]
    pub fn new() -> Self {
        Self {
            engines: HashMap::with_capacity(1024),
        }
    }
}

impl Default for UserEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl UserData for UserEngine {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        // Register a userscript scan engine.
        methods.add_method_mut(
            "register",
            |_, this: &mut UserEngine, (name, func): (String, Function)| {
                this.engines.insert(name, func);
                Ok(())
            },
        );

        // Run a scan against all userscript scan engines.
        methods.add_method("scan", |_, this, bytestring: mlua::String| {
            // When an engine returns true, it is pushed into this list.
            let mut matching_engines: Vec<String> = Vec::with_capacity(1024);

            // Call all scan engines sequentially.
            for (name, engine) in &this.engines {
                let result: bool = engine.call(&bytestring)?;
                if result {
                    matching_engines.push(name.clone());
                }
            }
            matching_engines.shrink_to_fit();
            Ok(matching_engines)
        });
    }
}

impl ApiObject for UserEngine {
    fn name(&self) -> &'static str {
        "user_engines"
    }
}
