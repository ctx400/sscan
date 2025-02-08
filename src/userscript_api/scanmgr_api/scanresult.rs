//! # Scan Result Userdata Types for [`ScanMgr`]
//!
//! This module defines the serializable data types that represent scan
//! results in a raw format.
//!
//! [`ScanMgr`]: super::ScanMgr

use crate::userscript_api::{fs_api::path_obj::PathObj, include::{Lua, LuaResult, LuaTable, LuaTableSequence, LuaUserData, LuaUserDataRef}};
use serde::Serialize;

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
    pub path: Option<PathObj>,
}

impl LuaUserData for DataItemResult {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this: &DataItemResult| Ok(this.name.clone()));
        fields.add_field_method_get("path", |_, this: &DataItemResult| Ok(this.path.clone()));
    }
}

/// Add a csv() method to the scan results table.
pub(super) async fn add_csv_method(lua: &Lua, results: &LuaTable) -> LuaResult<()> {
    let csv_method = lua.create_async_function(|_, (this, headers): (LuaTable, Option<bool>)| async move {
        // Create an iterator over the ScanResult table.
        let mut scan_results: LuaTableSequence<'_, LuaUserDataRef<ScanResult>> = this.sequence_values::<LuaUserDataRef<ScanResult>>();

        // This vector stores the CSV rows for serialization.
        let mut rows: Vec<String> = Vec::with_capacity(this.len()? as usize + 1);

        // If headers is true, add headers.
        if headers.is_some_and(|headers: bool| headers) {
            let headers: String = r#""Scan Engine","Item Name","Item Path""#.to_string();
            rows.push(headers);
        }

        // Serialize each row to CSV
        while let Some(Ok(scan_result)) = scan_results.next() {
            let row: String = format!(r#""{}","{}","{}""#, scan_result.engine, scan_result.item.name, scan_result.item.path.clone().unwrap_or_default().0.to_string_lossy());
            rows.push(row);
        }

        // Concat the rows vector to produce the final CSV.
        // Append a blank line at the end.
        let mut csv: String = rows.join("\n");
        csv.push('\n');

        // Return the CSV-serialized results.
        Ok(csv)
    })?;

    // Add the CSV method to the results table.
    results.set("csv", csv_method)?;
    Ok(())
}
