//! # Provides the Lua Userscript Environment
//!
//! The [`LuaVM`] actor provides a Lua 5.4 virtual machine, which in
//! turn provides the userscript environment. sscan is both configured
//! and driven by userscripts, and userscripts can also define custom
//! scan engines.
//!
//! ## Interacting with the Virtual Machine
//!
//! [`LuaVM`] is an asynchronous actor, meaning it runs the Lua virtual
//! machine on its own thread independently and has full control over
//! its own state. Interaction with the VM is done by message-passing.
//!
//! See the [`messages`] module to learn about the various types of
//! messages that can be sent to the VM to interact with it, along with
//! usage and code examples.
//!

pub mod error;
pub mod messages;

use crate::{
    actors::{queue::Queue, scanmgr::ScanMgr, user_engine::UserEngine},
    userscript_api::{about_api::AboutApi, help_system::HelpSystem},
};
use kameo::{actor::ActorRef, error::BoxError, mailbox::unbounded::UnboundedMailbox, Actor};
use messages::RegisterUserApi;
use mlua::{prelude::*, AppDataRefMut};

/// # An actor which hosts a Lua VM and userscript environment.
///
/// This actor instantiates and hosts the Lua 5.4 virtual machine which
/// in turn provides the userscript environment. The userscript
/// environment provides APIs into the inner workings of sscan for
/// customization, configuration, and defining custom scan engines.
pub struct LuaVM {
    /// The inner Lua 5.4 Virtual Machine
    vm: Lua,

    /// Reference to the [`Queue`] service
    queue: Option<ActorRef<Queue>>,

    /// Reference to the [`UserEngine`] service
    user_engine: Option<ActorRef<UserEngine>>,

    /// Reference to the [`ScanMgr`] service
    scanmgr: Option<ActorRef<ScanMgr>>,

    /// Extra "CLI-style" arguments.
    ///
    /// On startup, [`LuaVM`] will load this into a Lua array, `arg`,
    /// which userscripts can iterate to process their own command-line.
    ///
    args: Vec<String>,
}

/// # [`LuaVM`] is an actor.
///
/// This means that the virtual machine runs on its own thread and
/// communicates with other Rust components via message-passing. This
/// allows the virtual machine to run alongside other asynchronous
/// subsystems while maintaining owned mutable state without locks.
impl Actor for LuaVM {
    type Mailbox = UnboundedMailbox<Self>;

    async fn on_start(&mut self, lua_vm: ActorRef<Self>) -> Result<(), BoxError> {
        // Load the "CLI-style" args into table `arg`
        let args_table: LuaTable = self.vm.create_table()?;
        while let Some(arg) = self.args.pop() {
            args_table.push(arg)?;
        }
        self.vm.globals().set("arg", args_table)?;

        // Spawn other actors
        let queue: ActorRef<Queue> = Queue::spawn_with_size(lua_vm.downgrade(), 16384);
        let user_engine: ActorRef<UserEngine> =
            UserEngine::spawn_with_capacity(lua_vm.downgrade(), 128);
        let scanmgr: ActorRef<ScanMgr> = ScanMgr::spawn(
            lua_vm.downgrade(),
            queue.downgrade(),
            user_engine.downgrade(),
        );

        // Register auxillary userscript APIs
        lua_vm
            .tell(RegisterUserApi::with(HelpSystem::default()))
            .await?;
        lua_vm
            .tell(RegisterUserApi::with(AboutApi::default()))
            .await?;

        // Link all actors to self
        lua_vm.link(&queue).await;
        lua_vm.link(&user_engine).await;
        lua_vm.link(&scanmgr).await;

        // Store references to the other actors
        self.queue = Some(queue);
        self.user_engine = Some(user_engine);
        self.scanmgr = Some(scanmgr);

        // Create the warning buffer
        let warning_buffer: String = String::with_capacity(4096);
        self.vm.set_app_data(warning_buffer);

        // Set the warning function to emit warnings
        self.vm
            .set_warning_function(|lua: &Lua, msg: &str, incomplete: bool| {
                // Get the warning buffer
                let mut warning_buffer: AppDataRefMut<String> =
                    lua.app_data_mut().expect("warning buffer should exist");
                if incomplete {
                    warning_buffer.push_str(msg);
                } else {
                    eprintln!("[WARN] {warning_buffer}{msg}");
                    warning_buffer.clear();
                }
                Ok(())
            });
        Ok(())
    }
}

impl LuaVM {
    /// Spawn a new Lua virtual machine in default execution mode.
    #[must_use]
    pub fn spawn(args: Option<&[String]>) -> ActorRef<Self> {
        // Create the VM
        let mut lua_vm: Self = Self {
            vm: Lua::new(),
            queue: None,
            user_engine: None,
            scanmgr: None,
            args: Vec::new(),
        };

        // If "CLI-style" args were passed, insert them.
        if let Some(args) = args {
            lua_vm.args.reserve(args.len());
            args.clone_into(&mut lua_vm.args);
        }

        // Spawn LuaVM
        kameo::spawn(lua_vm)
    }

    /// Spawn a new Lua virtual machine with unsafe libraries loaded.
    ///
    /// This function spawns [`LuaVM`] in unsafe mode. This means unsafe
    /// libraries, such as `debug`, are loaded into Lua. *Be careful
    /// with this!*
    ///
    /// ## Safety
    ///
    /// Incorrect use of the Lua `debug` library or other unsafe
    /// libraries can cause *undefined behavior*, leading to panics or
    /// unpredictable side effects! Unsafe mode is only intended for
    /// advanced users, and testing purposes only. Production
    /// userscripts should *never* rely on any functionality provided by
    /// unsafe mode.
    #[must_use]
    pub unsafe fn spawn_unsafe(args: Option<&[String]>) -> ActorRef<Self> {
        // Create the VM
        let mut lua_vm: Self = Self {
            vm: Lua::unsafe_new(),
            queue: None,
            user_engine: None,
            scanmgr: None,
            args: Vec::new(),
        };

        // If "CLI-style" args were passed, insert them.
        if let Some(args) = args {
            lua_vm.args.reserve(args.len());
            args.clone_into(&mut lua_vm.args);
        }

        // Spawn LuaVM
        kameo::spawn(lua_vm)
    }
}
