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

// Enable pedantic linting
#![deny(missing_docs)]
#![deny(clippy::pedantic)]

/// # The distributed actors that make up sscan.
///
/// sscan makes use of an actor framework, [`kameo`], to allow many
/// of its components to run concurrently on their own threads, all the
/// while reducing complexity since each actor has ownership of its own
/// mutable state (no locks!)
///
/// The most fundamental actor is [`LuaVM`], which is the bread and
/// butter of sscan. It provides a Lua 5.4 virtual machine and
/// userscript environment, complete with APIs to customize, configure,
/// and control sscan's scan engines and services. Userscripts can also
/// define their own custom scan engines, extending sscan's baked-in
/// capabilities.
///
/// See each of the modules below to learn more about the actors that
/// power sscan.
///
/// [`LuaVM`]: crate::actors::lua_vm::LuaVM
pub mod actors {
    pub mod lua_vm;
    pub mod queue;
}
pub mod userscript_api;
