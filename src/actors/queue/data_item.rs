//! # Trait Definition for a [`DataItem`] and Default Implementations.
//!
//! This module defines the [`DataItem`] trait, which is any type the
//! [`Queue`] can accept, as well as a few impls of data items, such as
//! the [`RawDatum`] and [`File`] types.
//!
//! [`Queue`]: super::Queue

use std::path::PathBuf;
use super::error::QueueResult;

/// An item that can be enqueued in the [`Queue`].
///
/// Any type that implements [`DataItem`] can be enqueued in the
/// [`Queue`]. Two default implementations, [`RawDatum`] and [`File`],
/// have been provided for convienience.
///
/// [`Queue`]: super::Queue
pub trait DataItem where Self: Send {
    /// The human-friendly name of the data item.
    fn name(&self) -> String;

    /// The file path, if any, of the data item.
    ///
    /// This field is only relevant if the data item originated from a
    /// file. It is used both for logging and special, memory-efficient
    /// loading of files.
    fn path(&self) -> Option<PathBuf>;

    /// Consumes the [`DataItem`], returning its content.
    ///
    /// This method consumes a [`Box<dyn DataItem>`], returning its
    /// inner `name`, `path`, and `content` attributes. If the data item
    /// is a lazy type, then calling this function initiates any
    /// deferred processing steps.
    ///
    /// ## Errors
    ///
    /// Because data items support lazy processing, it is possible to
    /// successfully create a lazy data item, only for processing to
    /// fail after a call to [`DataItem::realize()`]. For this reason,
    /// realize returns a [`QueueResult`].
    fn realize(self: Box<Self>) -> QueueResult<(String, Option<PathBuf>, Vec<u8>)>;
}

/// # Raw, user-supplied data item.
///
/// Use this type when there is data to be enqueued that does not
/// originate from a file. For file data, it is better to use the
/// dedicated [`File`] type.
pub struct RawDatum {
    /// Human-friendly name of the data item.
    dname: String,

    /// The raw bytes comprising the data item.
    content: Vec<u8>,
}

impl RawDatum {
    /// Create a new, boxed [`RawDatum`].
    pub fn new<S,D>(name: &S, content: D) -> Box<Self>
    where
        S: ToString,
        D: Into<Vec<u8>>,
    {
        let name: String = name.to_string();
        let content: Vec<u8> = content.into();
        Box::new(Self { dname: name, content })
    }
}

impl DataItem for RawDatum {
    fn name(&self) -> String {
        self.dname.clone()
    }

    fn path(&self) -> Option<PathBuf> {
        None
    }

    fn realize(self: Box<Self>) -> QueueResult<(String, Option<PathBuf>, Vec<u8>)> {
        Ok((self.dname, None, self.content))
    }
}

/// # File Data Item
///
/// Use this type when there is data to be enqueued that originates from
/// a file. This type is designed to save on memory usage; see section
/// `Behavior` below.
///
/// ## Behavior
///
/// This type is lazy: on creation it stores just the path to
/// the file, and only once [`DataItem::realize()`] is called does it
/// actually load the file from disk.
///
/// If you need to eagerly load file contents into memory, consider
/// implementing trait [`DataItem`] on a custom file-based data item,
/// and then enqueueing that custom item instead.
pub struct File {
    /// Reference path to the file to be loaded.
    path: PathBuf,
}

impl File {
    /// Create a new, boxed [`File`] data item.
    ///
    /// This does not immediately load the file from disk. See section
    /// `Behavior` at the top of this page to learn more.
    pub fn new<P>(path: P) -> Box<Self>
    where
        P: Into<PathBuf>,
    {
        let path: PathBuf = path.into();
        Box::new(Self { path })
    }
}

impl DataItem for File {
    fn name(&self) -> String {
        if let Some(name) = self.path.file_name() {
            name.to_string_lossy().to_string()
        } else {
            "<unknown filename>".to_string()
        }
    }

    fn path(&self) -> Option<PathBuf> {
        Some(self.path.clone())
    }

    fn realize(self: Box<Self>) -> QueueResult<(String, Option<PathBuf>, Vec<u8>)> {
        let name: String = self.name();
        let path: PathBuf = self.path;
        let contents: Vec<u8> = std::fs::read(&path)?;
        Ok((name, Some(path), contents))
    }
}
