//! # Version and license info APIs
//!
//! This module adds API methods and data items to the Lua global scope
//! to retrieve version information in userscripts.
//!
//! ## Functions
//!
//! All functions in this module are accessible to userscripts in the
//! global scope.
//!
//! | Function | Returns | Description
//! | --- | :---: | --- |
//! | `license()` | nil | Pretty-print the license file. |
//! | `version()` | nil | Pretty-print version info about sscan. |
//!
//! ## Variables
//!
//! All variables in this module are added to a table called `about`,
//! which is accessible by userscripts as a global variable.
//!
//! | Name | Type | Description |
//! | --- | :---: | --- |
//! | `about.app_name` | string | The name of the crate at build time. |
//! | `about.authors` | string | The authors of sscan. |
//! | `about.description` | string | A short description of sscan. |
//! | `about.license` | string | The text of the crate's license. |
//! | `about.license_spdx` | string | The crate's SPDX license identifier. |
//! | `about.repository` | string | The URL to sscan's Github repository. |
//! | `about.version` | string | The build version of sscan. |
//!
//! ## Example
//!
//! ```lua
//! -- Print all version info and print license
//! version()
//! license()
//!
//! -- Access version and authors
//! print(about.version)
//! print(about.authors)
//! ```
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
const LICENSE: &str = include_str!("../../../LICENSE.md");

/// Registers the version and license info APIs with Lua.
///
/// This function registers the `about` table containing version and
/// license information, as well as two global convienience functions,
/// `version()` and `license()`, for pretty-printing version info to
/// stdout.
///
/// # Errors
///
/// Any errors returning from this function are Lua errors. If a Lua
/// error occurs, this is probably a bug and should be reported.
///
pub(crate) fn register_version_apis(lua: &Lua) -> LuaResult<()> {
    // Create an `about` table with version info keys.
    let about: LuaTable = lua.create_table()?;
    set_version_info(&about)?;

    // Create the version() and license() Lua functions.
    let version_func: LuaFunction = lua.create_function(move |_, ()| {
        print_version_info();
        Ok(())
    })?;
    let license_func: LuaFunction = lua.create_function(move |_, ()| {
        print_license();
        Ok(())
    })?;

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
        "{APP_NAME} v{VERSION} - {DESCRIPTION}\nRepository: {REPOSITORY}\nAuthors: {AUTHORS}\nLicense: {LICENSE_SPDX}"
    );
}

/// Prints the license file to stdout.
fn print_license() {
    println!("{LICENSE}");
}
