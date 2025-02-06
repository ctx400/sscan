use anyhow::Result;
use kameo::actor::ActorRef;
use mlua::{ObjectLike, Value};
use sscan::actors::lua_vm::{messages::EvalChunk, LuaVM};
use std::{
    backtrace::BacktraceStatus::Captured,
    io::{stdin, stdout, BufRead, Write},
};

/// Starts an interactive REPL. Never returns unless [`LuaVM`] exits.
pub async fn invoke(vm: &ActorRef<LuaVM>, nosplash: bool) {
    // Print the splash message
    if !nosplash {
        print_splash();
    }

    // Start REPL loop.
    let mut buffer: String = String::with_capacity(2048);
    loop {
        // Exit if the virtual machine dies.
        if !vm.is_alive() {
            return;
        }

        // Read a multiline Lua chunk terminated by a semicolon.
        read_chunk(&mut buffer);

        // Check if the `exit` keyword was passed
        if buffer == "exit" {
            break;
        }

        // Evaluate the chunk in the virtual machine
        match evaluate(vm, &buffer).await {
            Ok(value) => {
                print_result(value);
            }
            Err(error) => {
                print_error(&error.context("failed to evaluate chunk"));
            }
        }
    }
}

/// Try to pretty-print an error.
fn print_error(err: &anyhow::Error) {
    // Print the base error
    eprintln!("Error: {err}\n");

    // Create an iterator over all inner errors.
    // Skip the first error, as we've already printed it.
    let error_chain: std::iter::Skip<anyhow::Chain<'_>> = err.chain().skip(1);

    // Print all chained errors
    for err in error_chain {
        eprintln!("Caused by: {err}\n");
    }

    // Print a backtrace, if available
    if err.backtrace().status() == Captured {
        eprintln!("Backtrace:\n{}\n", err.backtrace());
    }
}

/// Try to pretty-print the result.
fn print_result(value: Value) {
    #[allow(clippy::match_wildcard_for_single_variants)] // invalid lint
    match value {
        Value::Nil => {}
        Value::Boolean(b) => println!("{b}"),
        Value::Integer(i) => println!("{i}"),
        Value::Number(n) => println!("{n}"),
        Value::String(s) => println!("{}", s.to_string_lossy()),
        Value::Table(t) => println!("<table@0x{:x}>", t.to_pointer() as usize),
        Value::Thread(t) => println!("<coroutine@0x{:x}>", t.to_pointer() as usize),
        Value::Function(f) => println!("<function@0x{:x}>", f.to_pointer() as usize),
        Value::UserData(u) => println!("{}", u.to_string().unwrap_or(format!("<userdata@{:x}>", u.to_pointer() as usize))),
        Value::LightUserData(l) => println!("<lightuserdata@0x{:x}>", l.0 as usize),
        Value::Error(e) => print_error(&anyhow::Error::from(*e)),
        _ => println!("<unknown@0x{}>", value.to_pointer() as usize),
    }
}

/// Evaluate the Lua expression and return a result.
async fn evaluate(vm: &ActorRef<LuaVM>, chunk: &str) -> Result<Value> {
    let eval_request: EvalChunk = chunk.into();
    Ok(vm.ask(eval_request).await?)
}

/// Reads a multiline Lua chunk, terminated by a semicolon.
fn read_chunk(buffer: &mut String) {
    // Flag to determine if the continuation prompt should be printed.
    let mut continuation: bool = false;

    // Clear the buffer before starting.
    buffer.clear();

    // An error printing the prompt is non-critical.
    let _ = print_prompt();

    while !buffer.trim().ends_with(';') {
        if continuation {
            // An error printing the prompt is non-critical.
            let _ = print_continuation();
        }
        if let Err(error) = stdin().lock().read_line(buffer) {
            // Create a human-friendly error message.
            let error: anyhow::Error = error.into();
            let error: anyhow::Error = error.context("could not read Lua chunk from stdin");

            // Print the error and reset the loop
            eprintln!("{error}");
            continue;
        }
        continuation = true;
    }

    // Trim the semicolon off of the end of the buffer.
    *buffer = buffer.trim().trim_end_matches(';').trim().into();
}

/// Prints a prompt message before input.
fn print_prompt() -> Result<()> {
    print!("sscan> ");
    stdout().lock().flush()?;
    Ok(())
}

/// Prints the continuation prompt.
fn print_continuation() -> Result<()> {
    print!("   ... ");
    stdout().lock().flush()?;
    Ok(())
}

/// Prints the [`SPLASH_MESSAGE`]
fn print_splash() {
    println!("{SPLASH_MESSAGE}");
}

/// The splash message printed by [`repl_print_splash()`].
const SPLASH_MESSAGE: &str = r"
@@@
@@@ Interactive REPL for sscan
@@@
@@@ Enter any valid multiline lua, terminated by a semicolon (;)
@@@ For help, use help(), to exit, use exit;
@@@
";
