//! # [`YaraEngine`] Actor Messages
//!
//! Message definitions for the [`YaraEngine`] actor. Other actors can
//! use these messages to communicate and interoperate with the YARA-X
//! scan engine.
//!
//! See each message for examples and usage information.
//!

use super::{
    result::{Error, MatchedRule},
    YaraEngine,
};
use kameo::message::{Context, Message};
use yara_x::{Compiler, Scanner};

/// Add a YARA rule to the [`YaraEngine`].
///
/// A request for [`YaraEngine`] to include a YARA rule source for
/// compilation. After adding all rules, and before scanning, an actor
/// must call [`CompileRules`] to build the sources into a format the
/// YARA-X scanner can understand.
///
/// # Reply
///
/// Expect no reply from [`YaraEngine`] after submitting this message.
///
/// # Example
///
/// ```
/// # use sscan::yara_engine::{messages::AddRule, YaraEngine};
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// // Create and span a YARA-X scan engine.
/// let engine = kameo::spawn(YaraEngine::default());
///
/// // Add and YARA rule for compilation.
/// let rule: String = r#"
///     rule HelloWorld {
///         meta:
///             author = "ctx400"
///             description = "Detects `Hello World`"
///         strings:
///             $a = "Hello World"
///         condition:
///             all of them
///     }
/// "#.to_string();
/// engine.tell(AddRule(rule)).await?;
/// # Ok(())
/// # }
/// ```
pub struct AddRule(pub String);

impl Message<AddRule> for YaraEngine {
    type Reply = ();

    async fn handle(&mut self, msg: AddRule, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        self.rules.push(msg.0);
    }
}

/// Compile all rules held by the [`YaraEngine`]
///
/// A request for [`YaraEngine`] to compile all added rules. Before
/// submitting this message, at lease one rule must be submitted using
/// the [`AddRule`] message.
///
/// # Reply
///
/// After submitting a [`CompileRules`] request, [`YaraEngine`] will
/// respond with `Result<(),`[`Error`]`>`
///
/// # Example
///
/// ```
/// # use sscan::yara_engine::{messages::*, YaraEngine};
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// // Create and span a YARA-X scan engine.
/// let engine = kameo::spawn(YaraEngine::default());
///
/// // Add and compile a YARA rule.
/// let rule: String = r#"
///     rule HelloWorld {
///         meta:
///             author = "ctx400"
///             description = "Detects `Hello World`"
///         strings:
///             $a = "Hello World"
///         condition:
///             all of them
///     }
/// "#.to_string();
/// engine.tell(AddRule(rule)).await?;
/// engine.tell(CompileRules).await?;
/// # Ok(())
/// # }
/// ```
pub struct CompileRules;

impl Message<CompileRules> for YaraEngine {
    type Reply = Result<(), Error>;

    async fn handle(&mut self, _: CompileRules, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        let mut compiler: Compiler = Compiler::new();
        for source in &self.rules {
            if let Err(error) = compiler.add_source(source.as_str()) {
                return Err(Error::compile_error(source, error));
            }
        }
        self.compiled = Some(compiler.build());
        Ok(())
    }
}

/// Scan the provided bytes using rules compiled by [`YaraEngine`].
///
/// A request for [`YaraEngine`] to scan a byte sequence with previously
/// compiled YARA rules. Before sending this message, an actor must
/// first add YARA rules with [`AddRule`], then compile all added rules
/// with [`CompileRules`].
///
/// # Reply
///
/// After submitting this message, expect [`YaraEngine`] to respond with
/// a `Result<Vec<`[`MatchedRule`]`>,`[`Error`]`>`.
///
/// # Example
///
/// ```
/// # use sscan::yara_engine::{messages::*, YaraEngine};
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// // Create and span a YARA-X scan engine.
/// let engine = kameo::spawn(YaraEngine::default());
///
/// // Add and compile a YARA rule.
/// let rule: String = r#"
///     rule HelloWorld {
///         meta:
///             author = "ctx400"
///             description = "Detects `Hello World`"
///         strings:
///             $a = "Hello World"
///         condition:
///             all of them
///     }
/// "#.to_string();
/// engine.tell(AddRule(rule)).await?;
/// engine.tell(CompileRules).await?;
///
/// // Scan against some data
/// let results = engine.ask(ScanBytes(b"abcHello Worldxyz".to_vec())).await?;
/// assert_eq!(results.len(), 1);
/// # Ok(())
/// # }
/// ```
pub struct ScanBytes(pub Vec<u8>);

impl Message<ScanBytes> for YaraEngine {
    type Reply = Result<Vec<MatchedRule>, Error>;

    async fn handle(&mut self, msg: ScanBytes, _: Context<'_, Self, Self::Reply>) -> Self::Reply {
        match self.compiled {
            Some(ref rules) => {
                let mut scanner = Scanner::new(rules);
                match scanner.scan(msg.0.as_slice()) {
                    Ok(results) => {
                        let mut output: Vec<MatchedRule> = Vec::new();
                        for rule in results.matching_rules() {
                            output.push(rule.into());
                        }
                        Ok(output)
                    }
                    Err(error) => Err(Error::scan_error(msg.0, error)),
                }
            }
            None => Err(Error::no_compiled_rules(msg.0)),
        }
    }
}
