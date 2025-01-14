//! # A scriptable file/process/network scanner
//!
//! **sscan** is a scriptable file, process, and network scanner.
//! Its high level of configurability is powered by userscripts which run in
//! an embeded [Lua](https://www.lua.org/) virtual machine.
//!
//! **sscani** is the interactive counterpart to sscan. In its current
//! state, *sscani is basically just a very primitive REPL!* However,
//! should this REPL prove useful, I may create a more robust version.
//!
//! # Usage
//!
//! Enter a Lua snippet in the REPL, terminated by a semicolon. sscan
//! will execute the snippet in the context of a userscript.
//!

#![warn(clippy::pedantic)]

use anyhow::{Error, Result};
use kameo::actor::ActorRef;
use mlua::Value as LuaValue;
use sscan::lua_vm::{
    messages::{CheckoutTable, CommitTable, EvaluateChunk, ExecuteChunk},
    LuaVM,
};
use std::io::stdin;

/// The default sscani rcfile. This is loaded into Lua as a string.
const RCFILE_DEFAULT: &str = include_str!("sscani/rc.default.lua");

/// The sscani help subsystem. Provides the Lua help function.
const LIB_SSCANI_HELP: &str = include_str!("sscani/sscani.help.lua");

/// The main sscani helper library. Should be loaded last.
const LIB_SSCANI_STD: &str = include_str!("sscani/sscani.std.lua");

#[tokio::main]
async fn main() -> Result<()> {
    // Define messages to be used with the REPL.
    let prompt_request: ExecuteChunk = ExecuteChunk::using("sscani.prompt()");
    let continuation_request: ExecuteChunk = ExecuteChunk::using("sscani.prompt_continue()");

    // Initialize the Lua virtual machine.
    let vm: ActorRef<LuaVM> = kameo::spawn(LuaVM::init()?);

    // Load and execute scanni's helper libraries.
    load_sscani_libs(&vm).await?;
    load_default_rcfile(&vm).await?;

    // Start REPL loop.
    loop {
        // Display the prompt
        vm.ask(prompt_request.clone()).await?;

        // Read a line of Lua.
        let mut buffer: String = String::with_capacity(2048);
        stdin().read_line(&mut buffer)?;

        // Very primitive support for line continuation.
        while !buffer.trim_end().ends_with(';') {
            // Display a continuation prompt.
            vm.ask(continuation_request.clone()).await?;

            // Read a new line from the buffer.
            stdin().read_line(&mut buffer)?;
        }
        // Trim the semicolon before execution.
        let snippet: &str = buffer.trim_end_matches(';');

        // Convert the snippet into an EvaluateChunk request.
        let eval_request: EvaluateChunk = EvaluateChunk::using(snippet);

        // Evaluate the Lua snippet. If a value is returned, print it.
        match vm.ask(eval_request).await {
            Ok(retval) => match retval {
                LuaValue::Nil => {}
                LuaValue::Boolean(b) => println!("{b}"),
                LuaValue::Function(f) => {
                    // Get and print the function name
                    let func_ptr: usize = f.to_pointer() as usize;
                    println!("<function @ {func_ptr:#x}>");
                }
                LuaValue::Integer(i) => println!("{i}"),
                LuaValue::Number(f) => println!("{f}"),
                LuaValue::String(s) => println!("{}", s.to_string_lossy()),
                _ => println!("<{}>", retval.type_name()),
            },
            Err(err) => {
                let err: Error = Error::new(err).context("unable to execute chunk");
                eprintln!("Error: {err:?}");
            }
        }
    }
}

async fn load_sscani_libs(vm: &ActorRef<LuaVM>) -> Result<()> {
    let chunk: String = format!("{LIB_SSCANI_HELP}\n{LIB_SSCANI_STD}");
    let exec_request: ExecuteChunk = ExecuteChunk::using(&chunk);
    Ok(vm.ask(exec_request).await?)
}

async fn load_default_rcfile(vm: &ActorRef<LuaVM>) -> Result<()> {
    // Checkout the sscani table from Lua globals.
    let checkout_request: CheckoutTable = CheckoutTable::with_name("sscani");
    let table: mlua::Table = vm.ask(checkout_request).await?;

    // Add the default rcfile and commit back to Lua.
    table.set("rc_default", RCFILE_DEFAULT)?;
    let commit_request: CommitTable = CommitTable::using(table, "sscani");
    Ok(vm.ask(commit_request).await?)
}
