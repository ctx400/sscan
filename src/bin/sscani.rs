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

use std::io::{stdin, stdout, Write};
use anyhow::Result;
use sscan::lua_api::LuaVM;

fn main() -> Result<()> {
    // Initialize the Lua virtual machine.
    let vm: LuaVM = LuaVM::init()?;
    vm.exec("version() print()")?;

    // Start REPL loop.
    loop {
        // Prompt for input
        print!("sscan> ");
        stdout().flush()?;

        // Read a line of Lua.
        let mut buffer: String = String::with_capacity(2048);
        stdin().read_line(&mut buffer)?;

        // Very primitive support for line continuation.
        while !buffer.trim_end().ends_with(";") {
            // Display a continuation prompt.
            print!("   ... ");
            stdout().flush()?;

            // Read a new line from the buffer.
            stdin().read_line(&mut buffer)?;
        }

        // Execute the Lua snippet. Trim the semicolon first.
        let snippet: &str = buffer.trim_end_matches(';');
        vm.exec(snippet)?;
    }
}
