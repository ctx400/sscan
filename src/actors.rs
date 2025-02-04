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

use crate::macros::impl_ping;
use lua_vm::LuaVM;
use queue::Queue;
use scanmgr::ScanMgr;
use user_engine::UserEngine;

pub mod lua_vm;
pub mod queue;
pub mod scanmgr;
pub mod user_engine;

/// # Ping an actor to ensure its message loop has started.
///
/// This message checks whether an actor's message loop is running by
/// awaiting a reply from the actor. It is automatically invoked by
/// [`lua_vm::messages::WaitStartup`].
///
/// ## Reply
///
/// Expect no reply from any actor.
pub struct Ping;

// Implement Ping on all actors
impl_ping!(LuaVM, Queue, ScanMgr, UserEngine);
