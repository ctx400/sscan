//! # A scriptable file/process/network scanner
//!
//! **sscan** is a scriptable file, process, and network scanner.
//! Its high level of configurability is powered by userscripts which run in
//! an embeded [Lua](https://www.lua.org/) virtual machine.
//!
//! Currently, scanning is provided by the
//! [YARA-X](https://virustotal.github.io/yara-x/) scan engine. YARA-X is a
//! Rust implementation of the original YARA scan engine. Additional scan
//! engines may be implemented or integrated in the future.
//!
//! The embedded Lua virtual machine is made possible by the
//! [mlua](https://crates.io/crates/mlua) crate.
//!

// Modules
mod lua;

// Scope Imports
use anyhow::Result;
use mlua::Lua;

/// Entrypoint for sscan.
fn main() -> Result<()> {
    // Initialize the Lua virtual machine.
    let lua: Lua = lua::init()?;

    // Run a test script to validate APIs
    lua.load("version() license()").exec()?;

    Ok(())
}
