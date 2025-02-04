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
    actors::{lua_vm::{error::LuaVmResult, LuaVM}, queue::Queue, scanmgr::ScanMgr, user_engine::UserEngine, Ping},
    userscript_api::ApiObject,
};
use kameo::{actor::ActorRef, message::{Context, Message}};

/// # Register a userscript API object with [`LuaVM`]
///
/// After defining an API object, it needs to be registered with the
/// virtual machine before userscripts can access the API. This message
/// instructs the virtual machine to load an [`ApiObject`] into Lua.
///
/// ## Reply
///
/// Expect a reply of [`LuaVmResult<()>`](LuaVmResult)
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
/// # impl LuaUserData for MyApi {}
/// # impl ApiObject for MyApi {
/// #   fn name(&self) -> &'static str {
/// #       "my_api"
/// #   }
/// # }
/// // Start LuaVM and register a userscript API
/// let vm: ActorRef<LuaVM> = LuaVM::spawn();
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
        msg.0.init_script(&self.vm)?;
        self.vm.globals().set(msg.0.name(), msg.0)?;
        Ok(())
    }
}

impl<A> RegisterUserApi<A>
where
    A: ApiObject,
{
    /// Create an API registration request with an [`ApiObject`]
    pub fn with(api: A) -> Self {
        Self(api)
    }
}

/// # Execute a Lua chunk in the virtual machine.
///
/// Requests for [`LuaVM`] to execute an arbitrary chunk of Lua code in
/// the context of the userscript environment.
///
/// ## Reply
///
/// Expect a reply of type [`LuaVmResult<()>`](LuaVmResult)
///
/// ## Example
///
/// ```
/// # use sscan::actors::lua_vm::{LuaVM, messages::ExecChunk};
/// # #[tokio::main]
/// # async fn main() {
/// // Spawn a new LuaVM actor
/// let vm = LuaVM::spawn();
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
        self.vm.load(msg.0).exec_async().await?;
        Ok(())
    }
}

impl<S> From<S> for ExecChunk
where
    S: ToString,
{
    fn from(value: S) -> Self {
        Self(value.to_string())
    }
}

/// # Evaluate a Lua expression and return the result.
///
/// Requests for [`LuaVM`] to evaluate a Lua expression in the context
/// of the userscript environment, returning any produced value.
///
/// ## Reply
///
/// Expect a reply of type [`LuaVmResult<mlua::Value>`](LuaVmResult)
///
/// ## Example
///
/// ```
/// # use sscan::actors::lua_vm::{LuaVM, messages::EvalChunk};
/// # #[tokio::main]
/// # async fn main() {
/// // Spawn a new LuaVM actor
/// let vm = LuaVM::spawn();
///
/// // Evaluate a Lua expression in the VM
/// let exec_request: EvalChunk = r#"
///     5 + 6
/// "#.into();
/// let result = vm.ask(exec_request).await.unwrap();
/// assert_eq!(result, mlua::Value::Integer(11));
/// # }
/// ```
pub struct EvalChunk(String);

impl Message<EvalChunk> for LuaVM {
    type Reply = LuaVmResult<mlua::Value>;

    async fn handle(&mut self, msg: EvalChunk, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        Ok(self.vm.load(msg.0).eval_async().await?)
    }
}

impl<S> From<S> for EvalChunk
where
    S: ToString,
{
    fn from(value: S) -> Self {
        Self(value.to_string())
    }
}

/// Send a warning message to [`LuaVM`].
///
/// A request to send a warning message to the virtual machine. Warning
/// messages are printed to stderr.
///
/// ## Reply
///
/// Expect no reply from the virtual machine.
///
/// ## Example
///
/// ```
/// # use sscan::actors::lua_vm::{LuaVM, messages::SendWarning};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let vm = LuaVM::spawn();
/// let warning = SendWarning::Complete("something went wrong".into());
/// vm.tell(warning).await?;
/// # Ok(())
/// # }
/// ```
pub enum SendWarning {
    /// Immediately flush the warning message.
    Complete(String),

    /// Buffer the warning message.
    ///
    /// The warning message will only be printed after another request
    /// of type [`SendWarning::Complete`].
    Incomplete(String),
}

impl Message<SendWarning> for LuaVM {
    type Reply = ();

    async fn handle(&mut self, msg: SendWarning, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        match msg {
            SendWarning::Complete(msg) => {
                self.vm.warning(msg, false);
            }
            SendWarning::Incomplete(msg) => {
                self.vm.warning(msg, true);
            }
        }
    }
}

/// # Waits until all actors have started up.
///
/// This should be called after [`LuaVM::spawn()`] to ensure all actors
/// have the time to start up before any userscripts try to run.
///
/// ## Reply
///
/// Expect no reply from the virtual machine.
///
/// ## Example
///
/// ```
/// # use sscan::actors::{LuaVM, messages::WaitStartup};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let vm = LuaVM::spawn();
/// vm.ask(WaitStartup).await?;
/// # Ok(())
/// # }
/// ```
pub struct WaitStartup;

impl Message<WaitStartup> for LuaVM {
    type Reply = ();

    async fn handle(&mut self, _: WaitStartup, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        // Get strongrefs to all actors.
        let queue: &ActorRef<Queue> = self.queue.as_ref().expect("infallible");
        let scanmgr: &ActorRef<ScanMgr> = self.scanmgr.as_ref().expect("infallible");
        let user_engine: &ActorRef<UserEngine> = self.user_engine.as_ref().expect("infallible");

        let _ = queue.ask(Ping).await;
        let _ = scanmgr.ask(Ping).await;
        let _ = user_engine.ask(Ping).await;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        actors::lua_vm::{
            messages::{EvalChunk, ExecChunk, RegisterUserApi},
            LuaVM,
        },
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
        let vm: ActorRef<LuaVM> = LuaVM::spawn();
        vm.ask(RegisterUserApi::with(MyApi)).await.unwrap();
    }

    /// This contains valid Lua, and should execute successfully.
    #[tokio::test]
    async fn should_exec_successfully() {
        // Create a LuaVM actor
        let vm: ActorRef<LuaVM> = LuaVM::spawn();

        // Create a chunk and execute it.
        let exec_request: ExecChunk = r#"
            assert(5 == 5)
        "#
        .into();
        vm.ask(exec_request).await.unwrap();
    }

    /// This contains a bad Lua assertion, and should panic on unwrap().
    #[tokio::test]
    #[should_panic]
    async fn should_fail_exec() {
        // Create a LuaVM actor
        let vm: ActorRef<LuaVM> = LuaVM::spawn();

        // Create a chunk and execute it.
        let exec_request: ExecChunk = r#"
            assert(5 == 4)
        "#
        .into();
        vm.ask(exec_request).await.unwrap();
    }

    /// This contains a valid Lua expression, so should return a result.
    #[tokio::test]
    async fn should_return_eval_result() {
        // Create a LuaVM actor
        let vm: ActorRef<LuaVM> = LuaVM::spawn();

        // Create an expression and execute it.
        let expr_request: EvalChunk = r#"
            5 + 6
        "#
        .into();
        let result: mlua::Value = vm.ask(expr_request).await.unwrap();
        assert_eq!(result, mlua::Value::Integer(11));
    }

    /// This contains a bad expression, so should panic.
    #[tokio::test]
    #[should_panic]
    async fn should_error_on_invalid_expr() {
        // Create a LuaVM actor
        let vm: ActorRef<LuaVM> = LuaVM::spawn();

        // Create an expression and execute it.
        // The table and key don't exist, so this should error.
        let expr_request: EvalChunk = r#"
            nonexistent_table.nonexistent_key
        "#
        .into();
        vm.ask(expr_request).await.unwrap();
    }
}
