//! # The distributed actors that make up sscan.
//!
//! sscan makes use of an actor framework, [`kameo`], to allow many
//! of its components to run concurrently on their own threads, all the
//! while reducing complexity since each actor has ownership of its own
//! mutable state (no locks!)
//!
//! The most fundamental actor is [`LuaVM`], which is the bread and
//! butter of sscan. It provides a Lua 5.4 virtual machine and
//! userscript environment, complete with APIs to customize, configure,
//! and control sscan's scan engines and services. Userscripts can also
//! define their own custom scan engines, extending sscan's baked-in
//! capabilities.
//!
//! See each of the modules below to learn more about the actors that
//! power sscan.
//!
//! [`LuaVM`]: crate::actors::lua_vm::LuaVM

pub mod lua_vm;
pub mod queue;
pub mod user_engine;
