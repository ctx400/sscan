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

pub mod actors;
pub(crate) mod macros;
pub mod userscript_api;
