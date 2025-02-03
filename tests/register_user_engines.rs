//! Tests if several userscript scan engines can be registered and used.
//!
//! This integration test checks whether it is possible to load and
//! register several userscript scan engines, then attempts to run scans
//! with these engines using the userscript API.
//!

use kameo::actor::ActorRef;
use sscan::actors::lua_vm::{
    messages::{EvalChunk, ExecChunk},
    LuaVM,
};

#[tokio::test]
async fn should_register_user_engines() {
    // Spawn the virtual machine
    let vm: ActorRef<LuaVM> = LuaVM::spawn();

    // Load some userscript scan engines the quick and dirty way
    let exec_request: ExecChunk = concat!(
        include_str!("register_user_engines/engine_alwaysfalse.lua"),
        include_str!("register_user_engines/engine_alwaystrue.lua"),
        include_str!("register_user_engines/engine_helloworld.lua"),
    )
    .into();
    vm.ask(exec_request)
        .await
        .expect("the concat'd files should have been clean for dirty loading");

    // register the scan engines
    let exec_request: ExecChunk = r#"
        user_engines:register("alwaysfalse", engine_alwaysfalse)
        user_engines:register("alwaystrue", engine_alwaystrue)
        user_engines:register("helloworld", engine_helloworld)
    "#
    .into();
    vm.ask(exec_request)
        .await
        .expect("should be a valid Lua chunk");

    // Run some test scans and verify the results.
    let result_1: mlua::Value = vm
        .ask(EvalChunk::from(
            r#"user_engines:scan("adosif8hhpauoiwehrsdblkjbasbldkjfhpaiouwhlfjd")"#,
        ))
        .await
        .unwrap();
    let result_2: mlua::Value = vm
        .ask(EvalChunk::from(
            r#"user_engines:scan("984wh9rauhwibehgdiHello Worldaodikjfakjskdhaj")"#,
        ))
        .await
        .unwrap();
    let result_3: mlua::Value = vm
        .ask(EvalChunk::from(
            r#"user_engines:scan("oaisjdhioq82ihwodjsnlfkjslkjdoaisjpdijadnljsd")"#,
        ))
        .await
        .unwrap();

    // Quick and dirty cast to Lua tables
    let result_1: &mlua::Table = result_1.as_table().unwrap();
    let result_2: &mlua::Table = result_2.as_table().unwrap();
    let result_3: &mlua::Table = result_3.as_table().unwrap();

    // Validate the correct number of expected results for each scan.
    assert_eq!(result_1.len().unwrap(), 1);
    assert_eq!(result_2.len().unwrap(), 2);
    assert_eq!(result_3.len().unwrap(), 1);
}
