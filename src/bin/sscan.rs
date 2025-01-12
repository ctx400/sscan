//! # A scriptable file/process/network scanner
//!
//! **sscan** is a scriptable file, process, and network scanner.
//! Its high level of configurability is powered by userscripts which run in
//! an embeded [Lua](https://www.lua.org/) virtual machine.
//!

#![warn(clippy::pedantic)]

// Scope Imports
use anyhow::Result;
use sscan::lua_api::LuaVM;

/// Entrypoint for sscan.
fn main() -> Result<()> {
    // Initialize the Lua virtual machine.
    let vm: LuaVM = LuaVM::init()?;

    // Run a test script to validate APIs
    vm.exec("version() license()")?;
    Ok(())
}
