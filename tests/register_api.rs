//! Tests if a userscript api can be defined, loaded, and called.
//!
//! This integration test checks whether it is possible to:
//!
//! - Define a userscript API,
//! - Define documentation for the userscript API,
//! - Register the API and its docs into the userscript environment,
//! - Call into the API from within Lua,
//! - Access help from Lua.
//!

use kameo::actor::ActorRef;
use sscan::{actors::lua_vm::{messages::{ExecChunk, RegisterUserApi}, LuaVM}, userscript_api::{help_system::{HelpSystem, HelpTopic}, include::UserData, ApiObject}};

/// A simple increment-only counter API.
pub struct CounterApi {
    /// Holds the state of the counter.
    counter: u64,
}

impl ApiObject for CounterApi {
    fn name(&self) -> &'static str {
        "counter"
    }
}

impl UserData for CounterApi {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        // Return the current value of the counter.
        fields.add_field_method_get("value", |_, this| {
            Ok(this.counter)
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        // Increment the counter by one.
        methods.add_method_mut("inc", |_, this, ()| {
            this.counter += 1;
            Ok(())
        });

        // Reset the counter to zero.
        methods.add_method_mut("reset", |_, this, ()| {
            this.counter = 0;
            Ok(())
        });
    }
}

impl Default for CounterApi {
    /// A default counter should be set to zero.
    fn default() -> Self {
        Self { counter: 0 }
    }
}

/// Userscript help system topic for the counter API.
pub struct CounterApiHelp;

impl HelpTopic for CounterApiHelp {
    fn name(&self) -> &'static str {
        "counter"
    }

    fn short_description(&self) -> &'static str {
        "A counter API that can only be incremented by one."
    }

    fn content(&self) -> &'static str {
        include_str!("register_api/topic.counter.md")
    }
}

#[tokio::test]
async fn should_register_api_and_help() -> anyhow::Result<()> {
    // Spawn a new userscript environment
    let vm: ActorRef<LuaVM> = kameo::spawn(LuaVM::default());

    // Create and register help articles
    let topic: CounterApiHelp = CounterApiHelp;
    let mut help_system: HelpSystem = HelpSystem::default();
    help_system.topic(Box::new(topic))?;

    // Register the help API
    vm.ask(RegisterUserApi::with(help_system)).await.unwrap();

    // Create and register the CounterAPI
    let counter: CounterApi = CounterApi::default();
    vm.ask(RegisterUserApi::with(counter)).await.unwrap();

    // Execute the Lua test script to ensure APIs are working.
    let exec_request: ExecChunk = include_str!("register_api/api_test.lua").into();
    vm.ask(exec_request).await.unwrap();

    // If we get this far, the test has passed.
    Ok(())
}
