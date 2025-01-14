//! # YARA-X Scan Engine Integration
//!
//! The [`YaraEngine`] actor provides byte scanning capabilities through
//! YARA-X. The actor accepts and compiles YARA rules, and executes scans
//! against byte sequences using those rules.
//!

// Module Imports
pub mod messages;
pub mod result;

// Scope Includes
use kameo::{mailbox::unbounded::UnboundedMailbox, Actor};
use yara_x::Rules;

/// Manages the lifecycle of the YARA-X scan engine.
///
/// YARA is an incredibly flexible, powerful tool for writing detection
/// rules. The [`YaraEngine`] actor instantiates and manages the
/// lifecycle of the YARA-X scanner, including the rules compiler.
///
/// # Usage
///
/// The recommended pattern is to instantiate the
/// [`System`](crate::system::System) actor first, then request an
/// [`ActorRef`](kameo::actor::ActorRef) to the YARA-X engine.
///
/// # Example
///
/// ```
/// # use sscan::{yara_engine::{YaraEngine, messages::AddRule}, system::{System, messages::GetActorYaraEngine}};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Instantate the System actor.
/// let system = kameo::spawn(System::default());
///
/// // Get a reference to the YaraEngine
/// let yara_engine = system.ask(GetActorYaraEngine).await?.unwrap();
///
/// // Add a YARA rule to the scan engine.
/// let rule = r#"
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
/// yara_engine.tell(AddRule(rule)).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct YaraEngine {
    /// Holds source rules for compilation.
    rules: Vec<String>,

    /// Holds compiled rules for scanning.
    compiled: Option<Rules>,
}

impl Actor for YaraEngine {
    type Mailbox = UnboundedMailbox<Self>;

    fn name() -> &'static str {
        "scan_engine__yara_x"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kameo::actor::ActorRef;
    use messages::{AddRule, CompileRules, ScanBytes};
    use result::MatchedRule;
    use std::collections::HashMap;

    /// Sample YARA rule matching on any occurrence of "Hello World"
    const HELLOWORLD_RULE: &str = r#"
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

    /// Should trigger a compilation error.
    const INVALID_RULE_SYNTAX: &str = r#"
        rule SyntaxError {
            meta:
                description = "Should trigger a compilation error"
                author = "ctx400"
            strings:
                $a: "Hello World"
            condition:
                all of them
        }
    "#;

    /// Byte sequence that should match the rule.
    const MATCHING_DATA: &[u8] = b"alksdjfhlkjashdflkjh-Hello World-laksjdfhlkjhadsf";

    /// Byte sequence that should not match the rule.
    const NONMATCHING_DATA: &[u8] = b"alksdjfhlkajsdhfl-Goodbye World-dfhlkajsdhflkj";

    #[tokio::test]
    async fn should_match_helloworld_rule() {
        // Create a YARA-X engine actor.
        let engine_ref: ActorRef<YaraEngine> = kameo::spawn(YaraEngine::default());

        // Load and compile a test rule.
        engine_ref
            .tell(AddRule(HELLOWORLD_RULE.to_string()))
            .await
            .unwrap();
        engine_ref.ask(CompileRules).await.unwrap();

        // Run a scan against some data
        let results: Vec<MatchedRule> = engine_ref
            .ask(ScanBytes(MATCHING_DATA.to_vec()))
            .await
            .unwrap();

        // Validate only one result returned
        assert_eq!(results.len(), 1);

        // Validate identifier correctly parsed
        assert_eq!(&results.first().unwrap().identifier, "HelloWorld");

        // Validate metadata properly extracted
        let metadata: &HashMap<String, String> = &results.first().unwrap().metadata;
        assert_eq!(metadata.get("author").unwrap(), "ctx400");
    }

    #[tokio::test]
    async fn should_not_match_helloworld_rule() {
        // Create a YARA-X engine actor.
        let engine_ref: ActorRef<YaraEngine> = kameo::spawn(YaraEngine::default());

        // Load and compile a test rule.
        engine_ref
            .tell(AddRule(HELLOWORLD_RULE.to_string()))
            .await
            .unwrap();
        engine_ref.ask(CompileRules).await.unwrap();

        // Run a scan against some data
        let results: Vec<MatchedRule> = engine_ref
            .ask(ScanBytes(NONMATCHING_DATA.to_vec()))
            .await
            .unwrap();

        // Validate only one result returned
        assert_eq!(results.len(), 0);
    }

    #[tokio::test]
    #[should_panic]
    async fn should_fail_compile_syntax() {
        // Create a YARA-X engine actor.
        let engine_ref: ActorRef<YaraEngine> = kameo::spawn(YaraEngine::default());

        // Load and compile a test rule.
        engine_ref
            .tell(AddRule(INVALID_RULE_SYNTAX.to_string()))
            .await
            .unwrap();

        // Should panic.
        engine_ref.ask(CompileRules).await.unwrap();
    }

    #[tokio::test]
    #[should_panic]
    async fn should_fail_no_compiled_rules() {
        // Create a YARA-X engine actor
        let engine_ref: ActorRef<YaraEngine> = kameo::spawn(YaraEngine::default());

        // Don't compile anything. Just try to scan. Should panic.
        engine_ref
            .ask(ScanBytes(MATCHING_DATA.to_vec()))
            .await
            .unwrap();
    }
}
