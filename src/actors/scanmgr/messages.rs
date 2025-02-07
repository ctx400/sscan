//! # Messages accepted by the [`ScanMgr`]
//!
//! As an asynchronous actor, the scan manager service communicates with
//! other actors and rust components via message passing. This module
//! defines the various messages that the scan manager service accepts,
//! their parameters, replies, and code examples.
//!
//! See each message to learn more about interacting with the scan
//! manager service, like invoking a scan operation.
//!

use crate::{actors::{
    lua_vm::messages::SendWarning,
    queue::messages::{Dequeue, GetLength},
    scanmgr::{
        error::{Error, ScanMgrResult},
        ScanMgr,
    },
    user_engine::messages::ScanBytes,
}, userscript_api::scanmgr_api::scanresult::{DataItemResult, ScanResult}};
use kameo::message::{Context, Message};
use std::path::PathBuf;

/// # Scan all data items in the queue against all active scan engines.
///
/// A request for [`ScanMgr`] to dequeue all [`DataItem`] objects in the
/// queue and test them against all activated scan engines.
///
/// ## Reply
///
/// Expect a reply of [`ScanMgrResult<Vec<ScanResult>>`].
///
/// ## Example
///
/// ```lua
/// scanmgr:scan()
/// ```
///
/// [`DataItem`]: crate::actors::queue::data_item::DataItem
pub struct InvokeScan;

impl Message<InvokeScan> for ScanMgr {
    type Reply = ScanMgrResult<Vec<ScanResult>>;

    async fn handle(&mut self, _: InvokeScan, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        // Get strongrefs to each dependent actor so they don't shutdown
        let Some(lua_vm) = self.lua_ref.upgrade() else {
            return Err(Error::NoLuaVm);
        };
        let Some(queue) = self.queue_ref.upgrade() else {
            return Err(Error::NoQueue);
        };
        let Some(user_engine) = self.user_engine_ref.upgrade() else {
            return Err(Error::NoUserEngine);
        };

        // Create a vector of ScanResult items
        let mut scan_results: Vec<ScanResult> = Vec::with_capacity(16384);

        // Get the current queue length
        while queue.ask(GetLength).await.expect("should be infallible") > 0 {
            // Dequeue an item or raise a warning on failure
            let (name, path, content) = match queue.ask(Dequeue).await {
                Ok((name, path, content)) => (name, path, content),
                Err(err) => {
                    let warning: String = format!("failed to load data item: {err}");
                    lua_vm
                        .tell(SendWarning::Complete(warning))
                        .await
                        .expect("should be infallible");
                    continue;
                }
            };

            // Scan the item against all user engines or raise a warning
            let Ok(results) = user_engine.ask(ScanBytes::from(content)).await else {
                let warning: String = format!("failed to scan data item `{name}`.\n  HINT: is the path accessible?\n        {path:?}");
                lua_vm
                    .tell(SendWarning::Complete(warning))
                    .await
                    .expect("should be infallible");
                continue;
            };

            // Create a ScanResult item for each user engine result
            for engine_name in results {
                let name: String = name.clone();
                let path: Option<PathBuf> = path.clone();
                let result = ScanResult {
                    engine: engine_name,
                    item: DataItemResult { name, path },
                };
                scan_results.push(result);
            }
        }
        Ok(scan_results)
    }
}
