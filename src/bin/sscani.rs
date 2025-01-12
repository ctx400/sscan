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

use anyhow::{Error, Result};
use mlua::Value as LuaValue;
use sscan::lua_api::LuaVM;
use std::io::stdin;

/// The sscani help subsystem. Provides help(['topic']).
const LIB_SSCANI_HELP: &str = include_str!("sscani/sscani.help.lua");

/// The main sscani helper library. Should be loaded last.
const LIB_SSCANI_STD: &str = include_str!("sscani/sscani.std.lua");

fn main() -> Result<()> {
    // Initialize the Lua virtual machine.
    let vm: LuaVM = LuaVM::init()?;

    // Load and execute scanni's helper libraries.
    vm.exec(LIB_SSCANI_HELP)?;
    vm.exec(LIB_SSCANI_STD)?;

    // Start REPL loop.
    loop {
        // Display the prompt
        vm.exec("sscani.prompt()")?;

        // Read a line of Lua.
        let mut buffer: String = String::with_capacity(2048);
        stdin().read_line(&mut buffer)?;

        // Very primitive support for line continuation.
        while !buffer.trim_end().ends_with(";") {
            // Display a continuation prompt.
            vm.exec("sscani.prompt_continue()")?;

            // Read a new line from the buffer.
            stdin().read_line(&mut buffer)?;
        }
        // Trim the semicolon before execution.
        let snippet: &str = buffer.trim_end_matches(';');

        // Evaluate the Lua snippet. If a value is returned, print it.
        match vm.eval(snippet) {
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
