//! # A scriptable file/process/network scanner
//!
//! **sscan** is a scriptable file, process, and network scanner.
//! Its high level of configurability is powered by userscripts which run in
//! an embeded [Lua](https://www.lua.org/) virtual machine.
//!

#![warn(clippy::pedantic)]


// Scope Imports
use anyhow::Result;
use kameo::actor::ActorRef;
use sscan::{lua_vm::messages::ExecuteChunk, system::{messages::GetActorLuaVM, System}};

/// Entrypoint for sscan.
#[tokio::main]
async fn main() -> Result<()> {
    let system_actor: ActorRef<System> = kameo::spawn(System::default());
    println!("STARTUP: Initialized system actor {}", system_actor.id());

    // Get the LuaVM actor and print version and license info
    if let Some(lua_vm) = system_actor.ask(GetActorLuaVM).await? {
        lua_vm.ask(ExecuteChunk::using("version() license()")).await?
    }

    println!("SHUTDOWN: Initiating system shutdown...");
    system_actor.stop_gracefully().await?;
    Ok(())
}
