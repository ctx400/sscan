//! # [`LuaVM`] Actor Messages
//!
//! Message definitions for the [`LuaVM`] actor. Other
//! actors can use these messages to communicate and interoperate with
//! the userscript environment.
//!
//! See each message for examples and usage information.
//!

use super::LuaVM;
use kameo::message::{Context, Message};
use mlua::prelude::*;

/// Execute a chunk of Lua code in the virutal machine.
///
/// A request for [`LuaVM`] to execute a provided Lua code snippet in
/// the context of the userscript environment.
///
/// # Reply
///
/// After submitting an [`ExecuteChunk`] request, expect a reply from
/// [`LuaVM`] of type `Result<(), mlua::Error>`.
///
/// # Example
///
/// ```
/// # use mlua::prelude::*;
/// # use sscan::lua_vm::{LuaVM, messages::ExecuteChunk};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create and spawn a userscript environment.
/// let vm = kameo::spawn(LuaVM::init()?);
///
/// // Execute some code in the userscript environment.
/// let exec_request = ExecuteChunk::using(r#"
///     print("Hello, world!")
/// "#);
/// vm.ask(exec_request).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ExecuteChunk {
    /// Lua script to execute in the virtual machine.
    chunk: String,
}

impl ExecuteChunk {
    /// Create an [`ExecuteChunk`] message using the provided Lua code.
    #[must_use]
    pub fn using(script: &str) -> Self {
        Self {
            chunk: script.to_owned(),
        }
    }
}

impl Message<ExecuteChunk> for LuaVM {
    type Reply = LuaResult<()>;

    async fn handle(
        &mut self,
        ExecuteChunk { chunk }: ExecuteChunk,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        self.0.load(chunk).exec()
    }
}

/// Evaluate a chunk of Lua code as an expression in the virtual machine.
///
/// This is similar to [`ExecuteChunk`], however it also requests that
/// [`LuaVM`] treat the provided chunk as an expression. As such, if the
/// expression yields a result, [`LuaVM`] returns the result to the
/// sender.
///
/// # Reply
///
/// After submitting an [`EvaluateChunk`] request, expect a reply from
/// [`LuaVM`] of type `Result<mlua::Value, mlua::Error>`.
///
/// # Example
///
/// ```
/// # use mlua::prelude::*;
/// # use sscan::lua_vm::{LuaVM, messages::EvaluateChunk};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create and spawn a userscript environment.
/// let vm = kameo::spawn(LuaVM::init()?);
///
/// // Evaluate an expression in the userscript environment.
/// let eval_request = EvaluateChunk::using(r#"
///     5 + 6
/// "#);
/// let reply: LuaValue = vm.ask(eval_request).await?;
/// assert_eq!(reply, LuaValue::Integer(11));
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct EvaluateChunk {
    /// Lua code to execute as an expression.
    chunk: String,
}

impl EvaluateChunk {
    /// Create an [`EvaluateChunk`] message using the provided Lua code.
    #[must_use]
    pub fn using(script: &str) -> Self {
        Self {
            chunk: script.to_owned(),
        }
    }
}

impl Message<EvaluateChunk> for LuaVM {
    type Reply = LuaResult<LuaValue>;

    async fn handle(
        &mut self,
        EvaluateChunk { chunk }: EvaluateChunk,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        self.0.load(chunk).eval()
    }
}

/// Checkout a table object from Lua globals.
///
/// A request for [`LuaVM`] to fetch a table object with the given name
/// from the current gloabls table.
///
/// NOTE! [`CheckoutTable`] and [`CommitTable`] requests are not atomic,
/// nor is there any locking! Another actor could potentially modify the
/// same table before the sender commits it back to Lua.
///
/// # Reply
///
/// After submitting a [`CheckoutTable`] request, expect a reply from
/// [`LuaVM`] of type `Result<mlua::Table, mlua::Error>`.
///
/// # Example
///
/// ```
/// # use mlua::prelude::*;
/// # use sscan::lua_vm::{LuaVM, messages::CheckoutTable};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create and spawn a userscript environment.
/// let vm = kameo::spawn(LuaVM::init()?);
///
/// // Checkout a table from lua globals.
/// let checkout_request = CheckoutTable::with_name("about");
/// let table: LuaTable = vm.ask(checkout_request).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct CheckoutTable {
    /// Name of the table to fetch.
    name: String,
}

impl CheckoutTable {
    /// Construct a [`CheckoutTable`] request with the provided name.
    #[must_use]
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

impl Message<CheckoutTable> for LuaVM {
    type Reply = LuaResult<LuaTable>;

    async fn handle(
        &mut self,
        CheckoutTable { name }: CheckoutTable,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        let table: LuaTable = self.0.globals().get(name)?;
        Ok(table)
    }
}

/// Commmit a table object into Lua globals.
///
/// A request for [`LuaVM`] to commit a table object with a given name
/// into the gloabls table.
///
/// NOTE! [`CheckoutTable`] and [`CommitTable`] requests are not atomic,
/// nor is there any locking! Another actor could potentially modify the
/// same table before the sender commits it back to Lua.
///
/// # Reply
///
/// After submitting a [`CommitTable`] request, expect a reply from
/// [`LuaVM`] of type `Result<(), mlua::Error>`.
///
/// # Example
///
/// ```
/// # use mlua::prelude::*;
/// # use sscan::lua_vm::{LuaVM, messages::{CheckoutTable, CommitTable}};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create and spawn a userscript environment.
/// let vm = kameo::spawn(LuaVM::init()?);
///
/// // Checkout a table from lua globals.
/// let checkout_request = CheckoutTable::with_name("about");
/// let table: LuaTable = vm.ask(checkout_request).await?;
///
/// // Make changes to the table
/// table.set("favorite_animal", "cat")?;
///
/// // Commit changes
/// let commit_request = CommitTable::using(table, "about");
/// vm.ask(commit_request).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct CommitTable {
    /// Destination table name.
    name: String,

    /// The table object to commit.
    table: LuaTable,
}

impl CommitTable {
    /// Construct a [`CommitTable`] message using specified table and name.
    #[must_use]
    pub fn using(table: LuaTable, name: &str) -> Self {
        Self {
            table,
            name: name.to_owned(),
        }
    }
}

impl Message<CommitTable> for LuaVM {
    type Reply = LuaResult<()>;

    async fn handle(
        &mut self,
        CommitTable { name, table }: CommitTable,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        self.0.globals().set(name, table)
    }
}
