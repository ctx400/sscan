use crate::userscript_api::help_system::HelpTopic;

/// Help topic definition for [`QueueApi`]
///
/// [`QueueApi`]: crate::userscript_api::queue::QueueApi
pub struct Topic;

impl HelpTopic for Topic {
    fn name(&self) -> &'static str {
        "queue"
    }

    fn short_description(&self) -> &'static str {
        "Queue up files and other data for scanning."
    }

    fn content(&self) -> &'static str {
        include_str!("topic.queue.txt")
    }
}
