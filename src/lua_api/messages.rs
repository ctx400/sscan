//! # Lua API Actor Messages
//!
//! This module implements message passing for the LuaVM actor. Other
//! actors can use these messages to communicate and interoperate with
//! the userscript environment.
//!

use super::LuaVM;
use kameo::message::{Context, Message};
use mlua::prelude::*;

/// Execute a chunk of Lua code in the virutal machine.
#[derive(Debug, Clone)]
pub struct ExecuteChunk {
    /// Lua script to execute in the virtual machine.
    chunk: String,
}

impl ExecuteChunk {
    /// Create an [`ExecuteChunk`] message using the provided Lua code.
    pub fn using(script: &str) -> Self {
        Self {
            chunk: script.to_owned(),
        }
    }
}

impl Message<ExecuteChunk> for LuaVM {
    type Reply = LuaResult<()>;

    async fn handle(&mut self, msg: ExecuteChunk, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        self.0.load(msg.chunk).exec()
    }
}

/// Evaluate a chunk of Lua code as an expression in the virtual machine.
#[derive(Debug, Clone)]
pub struct EvaluateChunk {
    /// Lua code to execute as an expression.
    chunk: String,
}

impl EvaluateChunk {
    /// Create an [`EvaluateChunk`] message using the provided Lua code.
    pub fn using(script: &str) -> Self {
        Self {
            chunk: script.to_owned(),
        }
    }
}

impl Message<EvaluateChunk> for LuaVM {
    type Reply = LuaResult<LuaValue>;

    async fn handle(&mut self, msg: EvaluateChunk, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        self.0.load(msg.chunk).eval()
    }
}

/// Checkout a table object from Lua globals.
#[derive(Debug, Clone)]
pub struct CheckoutTable {
    /// Name of the table to fetch.
    name: String,
}

impl CheckoutTable {
    /// Construct a [`CheckoutTable`] request with the provided name.
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

impl Message<CheckoutTable> for LuaVM {
    type Reply = LuaResult<LuaTable>;

    async fn handle(&mut self, msg: CheckoutTable, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        let table: LuaTable = self.0.globals().get(msg.name)?;
        Ok(table)
    }
}

/// Commit a table object into Lua globals.
#[derive(Debug, Clone)]
pub struct CommitTable {
    /// Destination table name.
    name: String,

    /// The table object to commit.
    table: LuaTable,
}

impl CommitTable {
    /// Construct a [`CommitTable`] message using specified table and name.
    pub fn using(table: LuaTable, name: &str) -> Self {
        Self {
            table, name: name.to_owned(),
        }
    }
}

impl Message<CommitTable> for LuaVM {
    type Reply = LuaResult<()>;

    async fn handle(&mut self, msg: CommitTable, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        self.0.globals().set(msg.name, msg.table)
    }
}
