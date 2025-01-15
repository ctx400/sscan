//! [`System`] Actor Messages
//!
//! Message definitions for the [`System`] actor. Other
//! actors can use these messages to communicate and interoperate with
//! the sscan system.
//!
//! See each message for examples and usage information.
//!

use super::System;
use crate::{lua_vm::LuaVM, yara_engine::YaraEngine};
use kameo::{
    actor::ActorRef,
    message::{Context, Message},
};

/// Request for an [`ActorRef`] to the [`LuaVM`] actor.
pub struct GetActorLuaVM;

impl Message<GetActorLuaVM> for System {
    type Reply = Option<ActorRef<LuaVM>>;

    async fn handle(&mut self, _: GetActorLuaVM, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        self.lua_vm.clone()
    }
}

/// Request for an [`ActorRef`] to the [`YaraEngine`] actor.
pub struct GetActorYaraEngine;

impl Message<GetActorYaraEngine> for System {
    type Reply = Option<ActorRef<YaraEngine>>;

    async fn handle(
        &mut self,
        _: GetActorYaraEngine,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        self.scan_engines.yara.clone()
    }
}
