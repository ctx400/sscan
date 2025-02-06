//! # Stores and retrieves interactive help topics
//!
//! The [`HelpSystem`] API stores a list of help topics, which userscripts
//! and interactive users can print by looking up the topic name. It
//! provides the Lua function `help 'topic'`, which prints detailed
//! help information on a given topic.
//!
//! ## Userscript API
//!
//! This is a userscript API. The API's functionality is registered with
//! the Lua virtual machine, where userscripts can call into it.
//!
//! ### API Usage Examples
//!
//! ```text
//! Usage: help()
//!   Print generic help information.
//!
//! Usage: help:topics()
//!   Print a list of all help topics.
//!
//! Usage: help 'topic'
//!   Print detailed help on a topic.
//! ```

pub mod error;

use crate::{
    macros::topics,
    userscript_api::ApiObject,
};
use error::Error;
use mlua::{ExternalError, UserData};
use std::collections::HashMap;

// List of Userscript API Topics
topics! {
    use HelpTopic about for "Build, version, and license information.";
    use HelpTopic fs for "Filesystem and directory handling methods.";
    use HelpTopic path for "Ergonomic file path maniuplation.";
    use HelpTopic queue for "Queue up files and other data for scanning.";
    use HelpTopic scanmgr for "Start a scan of all queued data items.";
    use HelpTopic user_engines for "Register custom userscript scan engines.";
}

/// # A help topic for userscript APIs.
///
/// Any type implementing this trait is eligible to be registered with
/// the [`HelpSystem`] as a help topic.
///
/// ## Example
///
/// ```
/// # use sscan::userscript_api::help_system::HelpTopic;
/// // Let's define a help topic.
/// struct MyHelpTopic;
///
/// // Provide the required help information.
/// impl HelpTopic for MyHelpTopic {
///     fn name(&self) -> &'static str {
///         "my_help_topic"
///     }
///
///     fn short_description(&self) -> &'static str {
///         "An example help topic for the help system."
///     }
///
///     fn content(&self) -> &'static str {
///         "
///         # MY HELP TOPIC #\n\
/// \
///         Here we can provide long-form, detailed help content for our topic. Each\
///         line should be no longer than 73 characters to prevent wrapping in most\
///         user's terminals, however, the content can span multiple paragraphs.\n\
/// \
///         The help content should be descriptive for end-users. For example, when\
///         documenting an API function, provide details on how to call the function,\
///         what arguments it takes, the expected return value(s), and any errors\
///         that might occur.\n\
/// \
///         For convenience, use the include_str!() function to add help content\
///         from a separate file instead of an inline string.
///         "
///     }
/// }
/// ```
///
/// Once we've registered our topic with the [`HelpSystem`], users and
/// userscripts can look up the help content using:
///
/// ```lua
/// help 'my_help_topic'
/// ```
pub trait HelpTopic
where
    Self: Send + Sync + 'static,
{
    /// # The unique name of the help topic.
    ///
    /// Required. The [`HelpSystem`] looks up help topics by name. The name
    /// can be any valid Lua string, however, for consistency, please
    /// read the [Formatting Advice](#formatting-advice) section below.
    ///
    /// **DO NOT** name your topic "`topics`", as this is a reserved name
    /// the Help System uses to list all other topics.
    ///
    /// ## Formatting Advice
    ///
    /// - A topic name ***must be unique!*** Otherwise it will overwrite
    ///   other help topics registered with the same name.
    /// - A topic name must be a valid Lua string.
    /// - Topic names should be at most 16 characters long.
    /// - Use all lowercase for topic names.
    /// - Use snake case to separate topic names.
    ///
    /// ## Examples of good topic names:
    ///
    /// - `myapi`
    /// - `myfunction`
    /// - `myapi_myfunction`
    /// - `topic_subtopic`
    /// - `my_help_topic`
    fn name(&self) -> &'static str;

    /// # A short, one-line description of the help topic.
    ///
    /// Required. The description should be a single-line string, and
    /// must be less than 50 characters long. The description is printed
    /// alongside the topic name when a user asks the [`HelpSystem`] to
    /// list all topics.
    ///
    /// A good help topic description is a short, but descriptive,
    /// synopsis of the content covered by the help topic.
    fn short_description(&self) -> &'static str;

    /// # The full help text for the topic.
    ///
    /// Required. This is the detailed help content the [`HelpSystem`]
    /// returns when a user looks up a help topic by name with
    /// `help 'topic'`. The format of the content is free-form,
    /// but try to write it in a way that is easy for end-users to
    /// digest.
    ///
    /// **NOTE**: Please keep all lines in the topic content shorter
    /// than 73 characters. This helps to prevent wrapping in most
    /// user's terminals.
    ///
    /// For convenience, the help topic content may be loaded using the
    /// [`include_str!`] macro, which allows the content to be stored in
    /// a separate file.
    fn content(&self) -> &'static str;
}
