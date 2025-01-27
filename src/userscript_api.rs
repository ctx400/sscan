//! # The Lua Userscript API
//!
//! One of the core features of sscan is its incredible flexibility
//! thanks to the integration of a Lua 5.4 virtual machine and the
//! userscript environment. Through this environment, scripts can
//! customize, configure, and control almost everything about sscan.
//! Furthermore, userscripts can define custom scan engines that
//! extend sscan beyond its baked-in capabilities.
//!
//! ## Module Structure
//!
//! The root of this module defines the traits necessary to expose
//! an API to the userscript environment, as well as the traits
//! necessary to register help topics with the userscript help system.
//!
//! The submodules of this module contain implementations of these
//! traits for each userscript API, along with both the Rust and Lua
//! documentation for those APIs.
//!
//! ## Developing Custom Userscript APIs
//!
//! To learn how to add custom APIs to the userscript environment, see
//! the [`ApiObject`] trait.
//!

pub mod help_system;
pub mod user_engine;
pub mod queue;
pub mod include {
    //! # Useful re-exports from other crates.
    //!
    //! This module provides re-exported types, traits, and functions
    //! from third-party crates where neccesary to implement the
    //! functionality of the userscript API.

    /// Exported from crate [`mlua`].
    pub use mlua::{UserData, UserDataFields, UserDataMethods};
}

use include::UserData;

/// # A userscript API object.
///
/// Any type implementing this trait is eligible to be registered with
/// [`LuaVM`] as a userscript API. A userscript API consists of one or more
/// data fields, functions, or methods, with which a userscript can
/// utilize to interact with a component of sscan.
///
/// Every API object must implement the [`UserData`] trait from crate
/// [`mlua`], and must be [`Send`] and `'static`.
///
/// # Example
///
/// ```
/// # use sscan::userscript_api::{ApiObject, include::*};
/// // Let's define a userscript API.
/// struct XkcdApi;
///
/// // We define our methods and fields here.
/// impl UserData for XkcdApi {
///     fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
///         methods.add_method("get_random_number", |_, _this: &XkcdApi, ()| {
///             Ok(4)
///         });
///     }
///
///     fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
///         fields.add_field_method_get("source", |_, _this: &XkcdApi| {
///             Ok("https://xkcd.com/221/")
///         });
///     }
/// }
///
/// // Finally, add the required metadata for an API object.
/// impl ApiObject for XkcdApi {
///     fn name(&self) -> &'static str {
///         "xkcd"
///     }
/// }
/// ```
///
/// Once we register our API, we can call it from Lua:
///
/// ```lua
/// random_number = xkcd:get_random_number()
/// assert(random_number == 4)
/// assert(xkcd.source == "https://xkcd.com/221/")
/// ```
///
/// [`LuaVM`]: crate::actors::lua_vm::LuaVM
pub trait ApiObject: UserData + Send + 'static {
    /// # The name of the API object, as visible from Lua
    ///
    /// `name` must be a valid Lua identifier. Valid Lua identifiers
    /// are any string of letters, digits, and underscores, but may not
    /// start with a digit. Identifiers are case-sensitive.
    ///
    /// ## Example Valid Lua Identifiers
    ///
    /// - `myfunc`
    /// - `my_func2`
    /// - `_myFunc_3`
    ///
    /// ## Example Invalid Lua Identifiers
    ///
    /// - `5myfunc`
    /// - `4_my_function`
    /// - `$myfunc`
    ///
    /// ## Don't use `_SPECIAL_IDENTIFIERS`!
    ///
    /// Try not to use any identifiers that start with an
    /// underscore followed by all uppercase letters, such as `_MYFUNC`,
    /// as Lua uses these sorts of identifiers internally for special
    /// purposes.
    ///
    /// ## Reserved Lua Keywords
    ///
    /// The following are reserved keywords in Lua 5.4, and may
    /// not be used as identifiers:
    ///
    /// ```lua
    /// and       break     do        else      elseif
    /// end       false     for       function  if
    /// in        local     nil       not       or
    /// repeat    return    then      true      until
    /// while
    /// ```
    ///
    /// ## Example
    ///
    /// ```
    /// # use sscan::userscript_api::{ApiObject, include::*};
    /// # struct MyApi;
    /// # impl UserData for MyApi {}
    /// impl ApiObject for MyApi {
    ///     fn name(&self) -> &'static str {
    ///         "my_api"
    ///     }
    /// }
    /// ```
    fn name(&self) -> &'static str;
}
