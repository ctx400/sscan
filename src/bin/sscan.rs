//! # A scriptable file/process/network scanner
//!
//! **sscan** is a scriptable file, process, and network scanner.
//! Its high level of configurability is powered by userscripts which run in
//! an embeded [Lua](https://www.lua.org/) virtual machine.
//!

// Scope Imports
use anyhow::Result;
use mlua::Lua;
use sscan::lua_api;

/// Entrypoint for sscan.
fn main() -> Result<()> {
    // Initialize the Lua virtual machine.
    let lua: Lua = lua_api::init()?;

    // Run a test script to validate APIs
    lua.load("version() license()").exec()?;

    Ok(())
}
