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
use sscan::system::System;
use std::time::Duration;

/// Entrypoint for sscan.
#[tokio::main]
async fn main() -> Result<()> {
    let system_actor: ActorRef<System> = kameo::spawn(System::default());
    println!("STARTUP: Initialized system actor {}", system_actor.id());

    tokio::time::sleep(Duration::from_millis(2000)).await;
    println!("SHUTDOWN: System actor stopped. Exiting sscan.");
    Ok(())
}
