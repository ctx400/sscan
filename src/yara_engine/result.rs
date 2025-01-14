//! [`Ok()`] and [`Err()`] types for YARA-X scans.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use yara_x::Rule;

/// Metadata about a YARA rule that matched during a scan operation.
#[derive(Serialize, Deserialize, Debug)]
#[must_use]
pub struct MatchedRule {
    pub identifier: String,
    pub namespace: String,
    pub metadata: HashMap<String, String>,
    pub tags: Vec<String>,
}

impl From<Rule<'_, '_>> for MatchedRule {
    fn from(value: Rule) -> Self {
        // Get the identifier and namespace
        let identifier: String = value.identifier().to_owned();
        let namespace: String = value.namespace().to_owned();

        // Get the rule's metadata
        let mut metadata: HashMap<String, String> = HashMap::new();
        for (key, value) in value.metadata() {
            let value = match value {
                yara_x::MetaValue::Bool(b) => b.to_string(),
                yara_x::MetaValue::Bytes(bs) => bs.to_string(),
                yara_x::MetaValue::Float(f) => f.to_string(),
                yara_x::MetaValue::Integer(i) => i.to_string(),
                yara_x::MetaValue::String(s) => s.to_string(),
            };
            metadata.insert(key.to_string(), value);
        }

        // Get the rules tags
        let mut tags: Vec<String> = Vec::new();
        for tag in value.tags() {
            tags.push(tag.identifier().to_string());
        }

        // Returned a new owned ScanResult
        Self {
            identifier,
            namespace,
            metadata,
            tags,
        }
    }
}

/// Comprehensive error type for [`YaraEngine`] errors.
#[derive(Error, Debug)]
#[must_use]
pub enum Error {
    #[error("failed to compile YARA rule(s): {code} - {title}\n\nFor Rule(s):\n{yara_src}\n\n{source}")]
    CompilationError {
        code: String,
        title: String,
        yara_src: String,
        source: yara_x::errors::CompileError,
    },
    #[error("the YARA-X scanner encountered an error: {source}\n\nFor byte(s):\n{bytes:?}")]
    ScanError {
        bytes: Vec<u8>,
        source: yara_x::errors::ScanError,
    },
    #[error("failed to launch scan: no compiled rules.\n\nFor byte(s):\n{bytes:?}\n\nHint: did you compile before launching a scan?")]
    NoCompiledRules {
        bytes: Vec<u8>
    },
}

impl Error {
    pub fn compile_error<S>(yara_src: &S, inner: yara_x::errors::CompileError) -> Self where S: ToString {
        let code: String = inner.code().to_owned();
        let title: String = inner.title().to_string();
        let yara_src: String = yara_src.to_string();
        Self::CompilationError { code, title, yara_src, source: inner }
    }

    pub fn scan_error(bytes: Vec<u8>, inner: yara_x::errors::ScanError) -> Self {
        Self::ScanError { bytes, source: inner }
    }

    pub fn no_compiled_rules(bytes: Vec<u8>) -> Self {
        Self::NoCompiledRules { bytes }
    }
}
