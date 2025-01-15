//! The sscan System Actor
//!
//! The [`System`] actor is responsible for initializing and
//! coordinating all of the moving parts of sscan. It spawns actors,
//! monitors their health, provides additional interoperational
//! capabilities, and helps sscan to gracefully recover in the event
//! of an actor panic.
//!

pub mod messages;

use kameo::{actor::ActorRef, error::BoxError, mailbox::unbounded::UnboundedMailbox, Actor};
use crate::{lua_vm::LuaVM, yara_engine::YaraEngine};

/// Oversees the smooth operation of sscan.
///
/// sscan provides many features and APIs through a system of
/// asynchronous actors. The [`System`] actor coordinates the
/// initialization and lifecycle of all other actors.
#[derive(Default)]
pub struct System {
    lua_vm: Option<ActorRef<LuaVM>>,
    scan_engines: ScanEngines,
}

impl Actor for System {
    type Mailbox = UnboundedMailbox<Self>;

    async fn on_start(&mut self, system: ActorRef<Self>) -> Result<(), BoxError> {
        // Start all actors
        self.lua_vm = Some(kameo::spawn(LuaVM::default()));
        self.scan_engines.yara = Some(kameo::spawn(YaraEngine::default()));

        // Register and link to all actors for health monitoring
        if let Some(ref lua_vm) = self.lua_vm {
            system.link(lua_vm).await
        }
        if let Some(ref yara_engine) = self.scan_engines.yara {
            system.link(yara_engine).await
        }

        Ok(())
    }

    fn name() -> &'static str {
        "sscan_system"
    }
}

/// Holds an [`ActorRef`] for each scan engine-type actor.
#[derive(Default)]
pub struct ScanEngines {
    /// The YARA-X scan engine.
    yara: Option<ActorRef<YaraEngine>>
}
