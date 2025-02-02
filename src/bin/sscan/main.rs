#![deny(clippy::pedantic)]

pub mod cli;
pub mod repl;

use anyhow::Result;
use clap::Parser;
use cli::{
    Action::{Interactive, Run},
    CliArgs,
};
use kameo::actor::ActorRef;
use sscan::actors::lua_vm::{messages::ExecChunk, LuaVM};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse commandline arguments
    let args: CliArgs = CliArgs::parse();

    // Initialize LuaVM and auxillary services.
    let vm: ActorRef<LuaVM> = match args.unsafe_mode {
        true => unsafe { LuaVM::spawn_unsafe() },
        false => LuaVM::spawn(),
    };
    vm.wait_startup().await;

    match args.action {
        Run { script } => {
            let exec_request: ExecChunk = load_script(script).await?.into();
            vm.ask(exec_request).await?;
        }
        Interactive {
            startup_script,
            nosplash,
        } => {
            if let Some(startup_script) = startup_script {
                let exec_request: ExecChunk = load_script(startup_script).await?.into();
                vm.ask(exec_request).await?;
            }
            repl::invoke_repl(&vm, nosplash).await;
        }
    }

    // Shut down all services
    vm.stop_gracefully().await?;
    vm.wait_for_stop().await;
    Ok(())
}

/// Load a userscript from disk into a [`String`].
async fn load_script<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let path: &Path = path.as_ref();
    let script = std::fs::read_to_string(path.canonicalize()?)?;
    Ok(script)
}
