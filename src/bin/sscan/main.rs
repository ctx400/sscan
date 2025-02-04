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
use sscan::actors::lua_vm::{messages::{ExecChunk, WaitStartup}, LuaVM};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse commandline arguments
    let args: Args = Args::parse();

    // Initialize LuaVM and auxillary services.
    let vm: ActorRef<LuaVM> = if args.unsafe_mode {
        unsafe { LuaVM::spawn_unsafe() }
    } else {
        LuaVM::spawn()
    };
    vm.wait_startup().await;
    vm.ask(WaitStartup).await?;

    match args.action {
        Run { script } => {
            let exec_request: ExecChunk = load_script(script)?.into();
            vm.ask(exec_request).await?;
        }
        Interactive {
            startup_script,
            nosplash,
        } => {
            if let Some(startup_script) = startup_script {
                let exec_request: ExecChunk = load_script(startup_script)?.into();
                vm.ask(exec_request).await?;
            }
            repl::invoke(&vm, nosplash).await;
        }
    }

    // Shut down all services
    vm.stop_gracefully().await?;
    vm.wait_for_stop().await;
    Ok(())
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
