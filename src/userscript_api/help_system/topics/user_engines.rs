use crate::userscript_api::help_system::HelpTopic;

/// Help topic definition for [`UserEngine`]
///
/// [`UserEngine`]: crate::userscript_api::user_engine::UserEngine
pub struct UserEngineHelp;

impl HelpTopic for UserEngineHelp {
    fn name(&self) -> &'static str {
        "user_engines"
    }

    fn short_description(&self) -> &'static str {
        "Register custom scan engines from userscripts."
    }

    fn content(&self) -> &'static str {
        include_str!("topic.user_engines.txt")
    }
}
