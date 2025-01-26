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
pub trait DataItem where Self: Send {
    /// The name of the data item.
    fn name(&self) -> String;

    /// The file path, if any, of the data item.
    fn path(&self) -> Option<PathBuf>;

    /// Consumes the DataItem, returning its content.
    fn realize(&self) -> QueueResult<(String, Option<PathBuf>, Vec<u8>)>;
}

pub struct RawDatum {
    name: String,
    content: Vec<u8>,
}

impl RawDatum {
    pub fn new<S,D>(name: &S, content: D) -> Box<Self>
    where
        S: ToString,
        D: Into<Vec<u8>>,
    {
        let name: String = name.to_string();
        let content: Vec<u8> = content.into();
        Box::new(Self { name, content })
    }
}

impl DataItem for RawDatum {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn path(&self) -> Option<PathBuf> {
        None
    }

    fn realize(&self) -> QueueResult<(String, Option<PathBuf>, Vec<u8>)> {
        Ok((self.name.clone(), None, self.content.clone()))
    }
}

pub struct File {
    path: PathBuf,
}

impl File {
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

    fn realize(&self) -> QueueResult<(String, Option<PathBuf>, Vec<u8>)> {
        let name: String = self.name();
        let path: PathBuf = self.path.clone();
        let contents: Vec<u8> = std::fs::read(&path)?;
        Ok((name, Some(path), contents))
    }
}
