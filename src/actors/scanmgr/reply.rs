//! # Scan Result Userdata Types for [`ScanMgr`]
//!
//! This module defines the serializable data types that represent scan
//! results in a raw format.
//!
//! [`ScanMgr`]: super::ScanMgr

use crate::userscript_api::include::LuaUserData;
use serde::Serialize;
use std::path::PathBuf;

/// Root return type for scan results.
#[derive(Serialize, Debug)]
pub struct ScanResult {
    /// Name of the engine that matched a [`DataItem`]
    ///
    /// [`DataItem`]: crate::actors::queue::data_item::DataItem
    pub engine: String,

    /// The [`DataItem`] that matched against a scan engine.
    ///
    /// [`DataItem`]: crate::actors::queue::data_item::DataItem
    pub item: DataItemResult,
}

impl LuaUserData for ScanResult {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("engine", |_, this: &ScanResult| Ok(this.engine.clone()));
        fields.add_field_method_get("item", |_, this: &ScanResult| Ok(this.item.clone()));
    }
}

/// Describes a [`DataItem`] match against a scan engine.
///
/// [`DataItem`]: crate::actors::queue::data_item::DataItem
#[derive(Serialize, Debug, Clone)]
pub struct DataItemResult {
    /// Name of the data item.
    pub name: String,

    /// Path of the data item, if applicable.
    pub path: Option<PathBuf>,
}

impl LuaUserData for DataItemResult {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this: &DataItemResult| Ok(this.name.clone()));
        fields.add_field_method_get("path", |_, this: &DataItemResult| Ok(this.path.clone()));
    }
}
