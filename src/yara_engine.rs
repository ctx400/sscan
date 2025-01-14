//! # YARA-X Scan Engine Integration
//!
//! This module defines the actors and messages for communicating with
//! the YARA-X scan engine.
//!

use std::collections::HashMap;
use kameo::{message::{Context, Message}, Actor};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use yara_x::{Compiler, Rule, Rules, Scanner};

/// Manages the lifecycle of the YARA-X scan engine.
#[derive(Actor, Default)]
pub struct YaraEngine {
    /// Holds source rules for compilation.
    rules: Vec<String>,

    /// Holds compiled rules for scanning.
    compiled: Option<Rules>,
}

/// Add a YARA rule to the [`YaraEngine`].
pub struct AddRule(String);

impl Message<AddRule> for YaraEngine {
    type Reply = ();

    async fn handle(&mut self, msg: AddRule, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        self.rules.push(msg.0);
    }
}

/// Compile all rules held by the [`YaraEngine`]
pub struct CompileRules;

impl Message<CompileRules> for YaraEngine {
    type Reply = Result<(), Error>;

    async fn handle(&mut self, _: CompileRules, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        let mut compiler: Compiler = Compiler::new();
        for source in &self.rules {
            if let Err(error) = compiler.add_source(source.as_str()) {
                return Err(Error::compile_error(source, error))
            }
        }
        self.compiled = Some(compiler.build());
        Ok(())
    }
}

/// Scan the provided bytes using rules compiled by [`YaraEngine`].
pub struct ScanBytes(Vec<u8>);

impl Message<ScanBytes> for YaraEngine {
    type Reply = Result<Vec<ScanResult>, Error>;

    async fn handle(&mut self, msg: ScanBytes, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        match self.compiled {
            Some(ref rules) => {
                let mut scanner = Scanner::new(rules);
                match scanner.scan(msg.0.as_slice()) {
                    Ok(results) => {
                        let mut output: Vec<ScanResult> = Vec::new();
                        for rule in results.matching_rules() {
                            output.push(rule.into());
                        }
                        Ok(output)
                    }
                    Err(error) => {
                        Err(Error::scan_error(msg.0, error))
                    }
                }
            },
            None => Err(Error::no_compiled_rules(msg.0))
        }
    }
}

/// Metadata about a YARA rule that matched during a scan operation.
#[derive(Serialize, Deserialize, Debug)]
#[must_use]
pub struct ScanResult {
    pub identifier: String,
    pub namespace: String,
    pub metadata: HashMap<String, String>,
    pub tags: Vec<String>,
}

impl From<Rule<'_, '_>> for ScanResult {
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

#[cfg(test)]
mod tests {
    use kameo::actor::ActorRef;

    use super::*;

    const TEST_RULE: &str = r#"
        rule HelloWorld {
            meta:
                description = "Detects `Hello World`"
                author = "ctx400"

            strings:
                $a = "Hello World"

            condition:
                all of them
        }
    "#;

    const TEST_DATA: &[u8] = b"alksdjfhlkjashdflkjh-Hello World-laksjdfhlkjhadsf";

    #[tokio::test]
    async fn scan_hello_world() {
        // Create a YARA-X engine actor.
        let engine_ref: ActorRef<YaraEngine> = kameo::spawn(YaraEngine::default());

        // Load and compile a test rule.
        engine_ref.tell(AddRule(TEST_RULE.to_string())).await.unwrap();
        engine_ref.tell(CompileRules).await.unwrap();

        // Run a scan against some data
        let results: Vec<ScanResult> = engine_ref.ask(ScanBytes(TEST_DATA.to_vec())).await.unwrap();

        // Validate only one result returned
        assert_eq!(results.len(), 1);

        // Validate identifier correctly parsed
        assert_eq!(&results.first().unwrap().identifier, "HelloWorld");

        // Validate metadata properly extracted
        let metadata: &HashMap<String, String> = &results.first().unwrap().metadata;
        assert_eq!(metadata.get("author").unwrap(), "ctx400");
    }
}
