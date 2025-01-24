pub mod cli;
pub mod repl;

use anyhow::Result;
use clap::Parser;
use cli::{
    Action::{Interactive, Run},
    CliArgs,
};
use kameo::actor::ActorRef;
use sscan::{
    actors::lua_vm::{
        messages::{ExecChunk, RegisterUserApi},
        LuaVM,
    },
    userscript_api::help_system::HelpSystem,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let _args: CliArgs = CliArgs::parse();
    let vm: ActorRef<LuaVM> = init_luavm().await?;

    match _args.action {
        Run { script } => {
            let exec_request: ExecChunk = load_script(script).await?.into();
            vm.ask(exec_request).await?;
        }
        Interactive { startup_script } => {
            if let Some(startup_script) = startup_script {
                let exec_request: ExecChunk = load_script(startup_script).await?.into();
                vm.ask(exec_request).await?;
            }
            repl::invoke_repl(&vm).await;
        }
    }

    vm.stop_gracefully().await?;
    vm.wait_for_stop().await;
    Ok(())
}

/// Initialize LuaVM and load APIs.
async fn init_luavm() -> Result<ActorRef<LuaVM>> {
    let vm: ActorRef<LuaVM> = kameo::spawn(LuaVM::default());

    // Register the HelpSystem API
    let help_api: HelpSystem = HelpSystem::new();
    vm.ask(RegisterUserApi::with(help_api)).await?;

    Ok(vm)
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
