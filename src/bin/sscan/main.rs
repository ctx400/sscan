#![deny(clippy::pedantic)]

pub mod cli;
pub mod repl;

use anyhow::Result;
use clap::Parser;
use cli::{
    Action::{Interactive, Run},
    Args,
};
use kameo::actor::ActorRef;
use sscan::actors::lua_vm::{
    messages::{ExecChunk, WaitStartup},
    LuaVM,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse commandline arguments
    let cli_args: Args = Args::parse();

    let vm: ActorRef<LuaVM> = match cli_args.action {
        Run { script, args } => {
            let vm: ActorRef<LuaVM> = init_vm(cli_args.unsafe_mode, &args).await?;
            let exec_request: ExecChunk = load_script(script)?.into();
            vm.ask(exec_request).await?;
            vm
        }
        Interactive {
            startup_script,
            nosplash,
            args,
        } => {
            let vm: ActorRef<LuaVM> = init_vm(cli_args.unsafe_mode, &args).await?;
            if let Some(startup_script) = startup_script {
                let exec_request: ExecChunk = load_script(startup_script)?.into();
                vm.ask(exec_request).await?;
            }
            repl::invoke(&vm, nosplash).await;
            vm
        }
    };

    // Shut down all services
    vm.stop_gracefully().await?;
    vm.wait_for_stop().await;
    Ok(())
}

/// Initialize the Lua virtual machine.
async fn init_vm(unsafe_mode: bool, args: &[String]) -> Result<ActorRef<LuaVM>> {
    let vm: ActorRef<LuaVM> = if unsafe_mode {
        unsafe { LuaVM::spawn_unsafe(Some(args)) }
    } else {
        LuaVM::spawn(Some(args))
    };
    vm.wait_startup().await;
    vm.ask(WaitStartup).await?;
    Ok(vm)
}

/// Load a userscript from disk into a [`String`].
fn load_script<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let path: &Path = path.as_ref();
    let script = std::fs::read_to_string(path.canonicalize()?)?;
    Ok(script)
}
