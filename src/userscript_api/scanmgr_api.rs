//! # Initate a scan against all enqueued data.
//!
//! The [`ScanMgrApi`] allows userscripts to initiate scan operations
//! of all enqueued data against all activated scan engines. It exposes
//! a higher-level API than invoking all the dequeueing and scanning
//! operations manually.
//!
//! ## Userscript API
//!
//! This is a userscript API. The API's functionality is registered with
//! the Lua virtual machine, where userscripts can call into it.
//!
//! ## Examples
//!
//! For full API documentation, launch sscan in interactive mode and
//! enter `help 'scanmgr'`, or see [`topics::scanmgr`].
//!
//! [`topics::scanmgr`]: crate::userscript_api::help_system::topics::scanmgr

pub mod scanresult;

use crate::{
    actors::scanmgr::{error::Error, messages::InvokeScan, ScanMgr},
    userscript_api::{
        include::{Lua, LuaExternalError, LuaTable, LuaUserDataRef},
        scanmgr_api::scanresult::{add_csv_method, ScanResult},
        ApiObject,
    },
};
use kameo::actor::WeakActorRef;
use mlua::UserData;
use scanresult::{add_json_method, add_ndjson_method};

/// # High-Level Scan Manager API
///
/// This [`ApiObject`] is exposed to the Lua userscript environment
/// and provides a high-level interface for initiating scan operations.
///
/// ## API Docs
///
/// To see detailed help for this API, launch sscan and call
/// `help 'scanmgr'`. Alternatively, the docs for this API are
/// available [here](super::help_system::topics::scanmgr)
pub struct ScanMgrApi(WeakActorRef<ScanMgr>);

impl ScanMgrApi {
    /// Create the userscript API object.
    #[must_use]
    pub fn new(scanmgr: WeakActorRef<ScanMgr>) -> Self {
        Self(scanmgr)
    }
}

impl UserData for ScanMgrApi {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method(
            "scan",
            |lua: Lua, this: LuaUserDataRef<ScanMgrApi>, ()| async move {
                // Get a strongref to the scan manager
                let Some(scanmgr) = this.0.upgrade() else {
                    return Err(Error::NoScanMgr.into_lua_err());
                };

                // Collect scan results.
                let raw_results: Vec<ScanResult> = scanmgr
                    .ask(InvokeScan)
                    .await
                    .map_err(LuaExternalError::into_lua_err)?;

                // Convert to a Lua table
                let results_table: LuaTable = lua.create_table()?;
                for result in raw_results {
                    results_table.push(result)?;
                }

                // Register result formatting methods
                add_csv_method(&lua, &results_table).await?;
                add_json_method(&lua, &results_table).await?;
                add_ndjson_method(&lua, &results_table).await?;

                // Return the results table
                Ok(results_table)
            },
        );
    }
}

impl ApiObject for ScanMgrApi {
    fn name(&self) -> &'static str {
        "scanmgr"
    }
}
