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

use crate::{
    actors::lua_vm::LuaVM,
    userscript_api::ApiObject,
};
use kameo::message::{Context, Message};

use super::error::LuaVmResult;

/// # Register a userscript API object with [`LuaVM`]
///
/// After defining an API object, it needs to be registered with the
/// virtual machine before userscripts can access the API. This message
/// instructs the virtual machine to load an [`ApiObject`] into Lua.
///
/// ## Reply
///
/// Expect a reply of [`LuaVmResult<()>`].
///
/// ## Example
///
/// ```
/// # use sscan::{
/// #     actors::lua_vm::{LuaVM, messages::RegisterUserApi},
/// #     userscript_api::{ApiObject, include::*},
/// # };
/// # use kameo::actor::ActorRef;
/// # #[tokio::main]
/// # async fn main() {
/// # struct MyApi;
/// # impl UserData for MyApi {}
/// # impl ApiObject for MyApi {
/// #   fn name(&self) -> &'static str {
/// #       "my_api"
/// #   }
/// # }
/// // Start LuaVM and register a userscript API
/// let vm: ActorRef<LuaVM> = kameo::spawn(LuaVM::default());
/// vm.ask(RegisterUserApi::with(MyApi)).await.unwrap();
/// # }
/// ```
pub struct RegisterUserApi<A>(A)
where
    A: ApiObject;

impl<A> Message<RegisterUserApi<A>> for LuaVM
where
    A: ApiObject,
{
    type Reply = LuaVmResult<()>;

    async fn handle(
        &mut self,
        msg: RegisterUserApi<A>,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        self.vm.globals().set(msg.0.name(), msg.0)?;
        Ok(())
    }
}

impl<A> RegisterUserApi<A>
where
    A: ApiObject,
{
    pub fn with(api: A) -> Self {
        Self(api)
    }
}

/// # Execute a Lua chunk in the virtual machine.
///
/// Requests for [`LuaVM`] to execute an arbitrary chunk of Lua code in
/// the context of the userscript environment. This operation returns no
/// value; see [`EvalChunk`] to evaluate and return the result of Lua
/// expressions.
///
/// ## Reply
///
/// Expect a reply of type [`LuaVmResult<()>`]
///
/// ## Example
///
/// ```
/// # use sscan::actors::lua_vm::{LuaVM, messages::ExecChunk};
/// # #[tokio::main]
/// # async fn main() {
/// // Spawn a new LuaVM actor
/// let vm = kameo::spawn(LuaVM::default());
///
/// // Execute a chunk of Lua in the VM
/// let exec_request: ExecChunk = r#"
///     x = 5
///     y = 6
///     z = x + y
///     assert(z == 11)
/// "#.into();
/// vm.ask(exec_request).await.unwrap();
/// # }
/// ```
pub struct ExecChunk(String);

impl Message<ExecChunk> for LuaVM {
    type Reply = LuaVmResult<()>;

    async fn handle(&mut self, msg: ExecChunk, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        self.vm.load(msg.0).exec()?;
        Ok(())
    }
}

impl<S> From<S> for ExecChunk where S: ToString {
    fn from(value: S) -> Self {
        Self(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        actors::lua_vm::{messages::{ExecChunk, RegisterUserApi}, LuaVM},
        userscript_api::ApiObject,
    };
    use kameo::actor::ActorRef;
    use mlua::{UserData, UserDataMethods};

    /// Any [`ApiObject`] should be able to be registered.
    #[tokio::test]
    async fn should_register_api() {
        // Create a simple userscript API
        struct MyApi;
        impl UserData for MyApi {
            fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
                methods.add_method("hello", |_, _this: &MyApi, ()| Ok("Hello World"));
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

    /// This contains valid Lua, and should execute successfully.
    #[tokio::test]
    async fn should_exec_successfully() {
        // Create a LuaVM actor
        let vm: ActorRef<LuaVM> = kameo::spawn(LuaVM::default());

        // Create a chunk and execute it.
        let exec_request: ExecChunk = r#"
            assert(5 == 5)
        "#.into();
        vm.ask(exec_request).await.unwrap();
    }

    /// This contains a bad Lua assertion, and should panic on unwrap().
    #[tokio::test]
    #[should_panic]
    async fn should_fail_exec() {
        // Create a LuaVM actor
        let vm: ActorRef<LuaVM> = kameo::spawn(LuaVM::default());

        // Create a chunk and execute it.
        let exec_request: ExecChunk = r#"
            assert(5 == 4)
        "#.into();
        vm.ask(exec_request).await.unwrap();
    }
}
