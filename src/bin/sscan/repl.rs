use anyhow::Result;
use kameo::actor::ActorRef;
use mlua::Value;
use sscan::actors::lua_vm::{messages::EvalChunk, LuaVM};
use std::{
    backtrace::BacktraceStatus::Captured,
    io::{stdin, stdout, BufRead, Write},
};

/// Starts an interactive REPL. Never returns unless LuaVM exits.
pub async fn invoke_repl(vm: &ActorRef<LuaVM>) {
    // Stores the current input chunk
    let mut buffer: String = String::with_capacity(2048);

    // Start REPL loop.
    loop {
        // Exit if the virtual machine dies.
        if !vm.is_alive() {
            return;
        }

        // Read a multiline Lua chunk terminated by a semicolon.
        repl_read(&mut buffer).await;

        // Evaluate the chunk in the virtual machine
        match repl_evaluate(vm, &buffer).await {
            Ok(value) => {
                repl_print(value).await;
            }
            Err(error) => {
                repl_print_error(error.context("failed to evaluate chunk")).await;
            }
        }
    }
}

/// Try to pretty-print an error.
async fn repl_print_error(err: anyhow::Error) {
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
        eprintln!("Backtrace:\n{}\n", err.backtrace())
    }
}

/// Try to pretty-print the result.
async fn repl_print(value: Value) {
    match value {
        Value::Nil => {}
        Value::Boolean(b) => println!("{b}"),
        Value::Integer(i) => println!("{i}"),
        Value::Number(n) => println!("{n}"),
        Value::String(s) => println!("{}", s.to_string_lossy()),
        Value::Table(t) => println!("<table@{:0x}>", t.to_pointer() as usize),
        Value::Thread(t) => println!("<coroutine@{:0x}>", t.to_pointer() as usize),
        Value::Function(f) => println!("<function@{:0x}>", f.to_pointer() as usize),
        Value::UserData(u) => println!("<userdata@{:0x}>", u.to_pointer() as usize),
        Value::LightUserData(l) => println!("<lightuserdata@{:0x}>", l.0 as usize),
        Value::Error(e) => eprintln!("{}", *e),
        _ => println!("<unknown>"),
    }
}

/// Evaluate the Lua expression and return a result.
async fn repl_evaluate(vm: &ActorRef<LuaVM>, chunk: &str) -> Result<Value> {
    let eval_request: EvalChunk = chunk.into();
    Ok(vm.ask(eval_request).await?)
}

/// Reads a multiline Lua chunk, terminated by a semicolon.
async fn repl_read(buffer: &mut String) {
    // Flag to determine if the continuation prompt should be printed.
    let mut continuation: bool = false;

    // Clear the buffer before starting.
    buffer.clear();

    // An error printing the prompt is non-critical.
    let _ = repl_print_prompt().await;

    while !buffer.trim().ends_with(';') {
        if continuation {
            // An error printing the prompt is non-critical.
            let _ = repl_print_continuation().await;
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
    *buffer = buffer.trim().trim_end_matches(';').to_owned();
}

/// Prints a prompt message before input.
async fn repl_print_prompt() -> Result<()> {
    print!("sscan> ");
    stdout().lock().flush()?;
    Ok(())
}

/// Prints the continuation prompt.
async fn repl_print_continuation() -> Result<()> {
    print!("   ... ");
    stdout().lock().flush()?;
    Ok(())
}
