//! Adds version information APIs to Lua.
//!
//! This module adds API methods and data items to the Lua global scope
//! to retrieve version information in userscripts.
//!

// Scope Includes
use mlua::prelude::*;

/// The name of this crate.
const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// The build version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The crate authors.
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

/// A short description of the crate.
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Source repository for the crate.
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

/// SPDX identifier for the crate's license.
const LICENSE_SPDX: &str = env!("CARGO_PKG_LICENSE");

/// The full text of the crate's license.
const LICENSE: &str = include_str!("../../LICENSE.md");


/// Registers the version and license info APIs.
pub fn register_version_apis(lua: &Lua) -> LuaResult<()> {
    // Create an `about` table with version info keys.
    let about: LuaTable = lua.create_table()?;
    set_version_info(&about)?;

    // Create the version() and license() Lua functions.
    let version_func: LuaFunction = lua.create_function(move |_, ()| Ok(print_version_info()))?;
    let license_func: LuaFunction = lua.create_function(move |_, ()| Ok(print_license()))?;

    // Register the table and functions in the global scope.
    let globals: LuaTable = lua.globals();
    globals.set("about", about)?;
    globals.set("version", version_func)?;
    globals.set("license", license_func)
}

/// Adds version information variables to a Lua table.
fn set_version_info(table: &LuaTable) -> LuaResult<()> {
    table.set("app_name", APP_NAME)?;
    table.set("version", VERSION)?;
    table.set("authors", AUTHORS)?;
    table.set("description", DESCRIPTION)?;
    table.set("repository", REPOSITORY)?;
    table.set("license_spdx", LICENSE_SPDX)?;
    table.set("license", LICENSE)
}

/// Pretty-prints version information to stdout.
fn print_version_info() {
    println!(
        "{} v{} - {}\nRepository: {}\nAuthors: {}\nLicense: {}",
        APP_NAME,
        VERSION,
        DESCRIPTION,
        REPOSITORY,
        AUTHORS,
        LICENSE_SPDX,
    );
}

/// Prints the license file to stdout.
fn print_license() {
    println!("{}", LICENSE);
}
