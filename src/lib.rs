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
//! ## Getting Started
//!
//! First of all, install sscan by running the following:
//!
//! ```bash
//! cargo install --locked sscan
//! ```
//!
//! To try out sscan interactively, run:
//!
//! ```bash
//! sscan interactive
//! ```
//!
//! Or, if you've already created a Lua userscript, run:
//!
//! ```bash
//! sscan run myscript.lua
//! ```
//!
//! The commandline arguments can be abbreviated, as long as they are
//! unambiguous. For example, `sscan int` or `sscan i` will start an
//! interactive session, just like the full command.
//!
//! ## Getting Help
//!
//! To access the built-in help system, call (from Lua):
//!
//! ```lua
//! help()            -- View general help overview
//! help:topics()     -- List all available help topics
//! help 'topic_name' -- View detailed help on a topic.
//! ```
//!
//! Help topics are also available in the [`topics`] module.
//!
//! [`topics`]: crate::userscript_api::help_system::topics

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
    pub mod user_engine;
}
pub(crate) mod macros;
pub mod userscript_api;
