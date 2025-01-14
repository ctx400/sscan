//! # [`YaraEngine`] Actor Messages

use kameo::message::{Context, Message};
use yara_x::{Compiler, Scanner};
use super::{Error, MatchedRule, YaraEngine};

/// Add a YARA rule to the [`YaraEngine`].
pub struct AddRule(pub String);

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
                    Err(error) => {
                        Err(Error::scan_error(msg.0, error))
                    }
                }
            },
            None => Err(Error::no_compiled_rules(msg.0))
        }
    }
}
