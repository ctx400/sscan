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
//! ## Usage
//!
//! There are two binaries included with sscan: `sscan` and `sscani`.
//! The main binary is `sscan`, the command-line file/process/network
//! scanner tool.
//!
//! The `sscani` binary is a *very* primitive interactive Lua REPL
//! primarily intended for light testing or debugging. It can accept
//! multiline Lua snippets terminated by semicolons, which execute in
//! the same context as userscripts do in `sscan`.
//!

// Enable pedantic linting
#![warn(clippy::pedantic)]

// Modules
pub mod lua_api;
