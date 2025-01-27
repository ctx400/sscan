//! # Add or remove items from the global scan queue
//!
//! The [`QueueApi`] allows userscripts to enqueue data items with the
//! [global scan queue]. As such, it is possible to programmatically
//! enqueue files and other data, rather than just providing a static
//! file list.
//!
//! ## Userscript API
//!
//! This is a userscript API. The API's functionality is registered with
//! the Lua virtual machine, where userscripts can call into it.
//!
//! ## API Usage Examples
//!
//! **For full API documentation, launch sscan in interactive mode and
//!   enter `help 'queue'`.**
//!
//! ```lua
//! queue:add_file 'path/to/file'
//!     Enqueue a file for scanning.
//!
//! queue:add_dir(dir, recursive=false)
//!     Enqueue all files in a directory, optionally using recursion.
//!
//! Usage: queue:add_raw(name, data)
//!     Enqueue a raw Lua string/bytestring for scanning.
//!
//! Usage: queue:dequeue()
//!     Pull the data item at the front of the queue.
//! ```
//!
//! [global scan queue]: crate::actors::queue::Queue

use kameo::actor::WeakActorRef;
use crate::actors::queue::Queue;

pub struct QueueApi {
    /// WeakRef to the global queue actor.
    gq: WeakActorRef<Queue>,
}
