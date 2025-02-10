//! # Windows Implementation of the Read Process Memory APIs.

use std::{ffi::c_void, mem::size_of};
use windows::Win32::{Foundation::HANDLE, System::{Memory::{VirtualQueryEx, MEMORY_BASIC_INFORMATION}, Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ}}};
use crate::userscript_api::{include::{LuaUserData, LuaExternalError}, ApiObject};

/// # The Process Memory Scanning Userscript API
pub struct ProcessApi;

impl LuaUserData for ProcessApi {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method("maps", |_, _, pid: u32| async move {
            // Get a process handle to the target PID.
            let handle: HANDLE = unsafe {
                OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, false, pid)
            }.map_err(LuaExternalError::into_lua_err)?;

            // Set up the receiving meminfo struct and base address.
            let mut mbi: MEMORY_BASIC_INFORMATION = MEMORY_BASIC_INFORMATION::default();
            let mut base_address: *const c_void = std::ptr::null();

            // Stores all received meminfo from the Win32 API.
            let mut mbi_items: Vec<MEMORY_BASIC_INFORMATION> = Vec::with_capacity(4096);

            // Query Virtual Memory Pages.
            while unsafe {
                VirtualQueryEx(handle, Some(base_address), (&mut mbi) as *mut MEMORY_BASIC_INFORMATION, size_of::<MEMORY_BASIC_INFORMATION>())
            } != 0 {
                println!("{mbi:?}");

                base_address = base_address.wrapping_add(mbi.RegionSize);
                mbi_items.push(mbi);
            };

            println!("return a Vec<MemoryMap> derived from all MEMORY_BASIC_INFORMATION structs");
            Ok(())
        });
    }
}

impl ApiObject for ProcessApi {
    fn name(&self) -> &'static str {
        "process"
    }
}
