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
use sscan::lua_api::{messages::ExecuteChunk, LuaVM};

/// Entrypoint for sscan.
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the Lua virtual machine.
    let vm_actor: ActorRef<LuaVM> = kameo::spawn(LuaVM::init()?);

    // Execute a test script on the virtual machine.
    let exec_msg: ExecuteChunk = ExecuteChunk::using("version() license()");
    vm_actor.ask(exec_msg).await?;
    Ok(())
}
