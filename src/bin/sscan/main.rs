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
use sscan::{actors::lua_vm::{
    messages::{EvalChunk, ExecChunk, WaitStartup},
    LuaVM,
},userscript_api::include::LuaValue};
use std::{path::Path, process::ExitCode};

#[tokio::main]
async fn main() -> Result<ExitCode> {
    // Parse commandline arguments
    let cli_args: Args = Args::parse();

    let (vm, exit_code): (ActorRef<LuaVM>, ExitCode) = match cli_args.action {
        Run { script, args } => {
            let vm: ActorRef<LuaVM> = init_vm(cli_args.unsafe_mode, &args).await?;
            let exec_request: EvalChunk = load_script(script)?.into();
            let return_val: LuaValue = vm.ask(exec_request).await?;

            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let exit_code: ExitCode = match return_val {
                LuaValue::Integer(rc) => ExitCode::from(rc as u8),
                LuaValue::Number(rc) => ExitCode::from(rc as u8),
                _ => ExitCode::SUCCESS,
            };
            (vm, exit_code)
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
            (vm, ExitCode::SUCCESS)
        }
    };

    // Shut down all services
    vm.stop_gracefully().await?;
    vm.wait_for_stop().await;
    Ok(exit_code)
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
