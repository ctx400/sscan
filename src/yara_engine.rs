//! # YARA-X Scan Engine Integration
//!
//! This module defines the actors and messages for communicating with
//! the YARA-X scan engine.
//!

pub mod messages;
pub mod result;

use kameo::Actor;
use yara_x::Rules;

/// Manages the lifecycle of the YARA-X scan engine.
#[derive(Actor, Default)]
pub struct YaraEngine {
    /// Holds source rules for compilation.
    rules: Vec<String>,

    /// Holds compiled rules for scanning.
    compiled: Option<Rules>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use kameo::actor::ActorRef;
    use messages::{AddRule, CompileRules, ScanBytes};
    use result::MatchedRule;
    use std::collections::HashMap;

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
        engine_ref
            .tell(AddRule(TEST_RULE.to_string()))
            .await
            .unwrap();
        engine_ref.tell(CompileRules).await.unwrap();

        // Run a scan against some data
        let results: Vec<MatchedRule> =
            engine_ref.ask(ScanBytes(TEST_DATA.to_vec())).await.unwrap();

        // Validate only one result returned
        assert_eq!(results.len(), 1);

        // Validate identifier correctly parsed
        assert_eq!(&results.first().unwrap().identifier, "HelloWorld");

        // Validate metadata properly extracted
        let metadata: &HashMap<String, String> = &results.first().unwrap().metadata;
        assert_eq!(metadata.get("author").unwrap(), "ctx400");
    }
}
