//! # Messages Accepted by [`LuaVM`]
//!
//! As an asynchronous actor, the Lua virtual machine communicates with
//! other actors and rust components via message passing. This module
//! defines the messages that the VM accepts, their parameters, replies,
//! and code examples.
//!
//! See each message to learn more about interacting with and
//! controlling the virtual machine.
//!

use crate::{userscript_api::ApiObject, actors::lua_vm::LuaVM};
use kameo::message::{Context, Message};
use mlua::prelude::*;

/// Register a userscript API object with [`LuaVM`]
///
/// After defining an API object, it needs to be registered with the
/// virtual machine before userscripts can access the API. This message
/// instructs the virtual machine to load an [`ApiObject`] into Lua.
pub struct RegisterUserApi<A>(A) where A: ApiObject;

impl<A> Message<RegisterUserApi<A>> for LuaVM where A: ApiObject {
    type Reply = LuaResult<()>;

    async fn handle(&mut self, msg: RegisterUserApi<A>, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        self.vm.globals().set(msg.0.name(), msg.0)
    }
}

impl<A> RegisterUserApi<A> where A: ApiObject {
    pub fn with(api: A) -> Self {
        Self(api)
    }
}

#[cfg(test)]
mod tests {
    use kameo::actor::ActorRef;
    use mlua::{UserData, UserDataMethods};
    use crate::{actors::lua_vm::{messages::RegisterUserApi, LuaVM}, userscript_api::ApiObject};

    /// Any [`ApiObject`] should be able to be registered.
    #[tokio::test]
    async fn should_register_api() {
        // Create a simple userscript API
        struct MyApi;
        impl UserData for MyApi {
            fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
                methods.add_method("hello", |_, _this: &MyApi, ()| {
                    Ok("Hello World")
                });
            }
        }
        impl ApiObject for MyApi {
            fn name(&self) -> &'static str {
                "myapi"
            }
        }

        // Register the API with LuaVM
        let vm: ActorRef<LuaVM> = kameo::spawn(LuaVM::default());
        vm.ask(RegisterUserApi::with(MyApi)).await.unwrap();
    }
}