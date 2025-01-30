//! # Helper macro definitions for sscan.
//!
//! This module defines various macros as convienience functions, both
//! for internal library usage and for external usage.
//!

/// # Helper to add userscript API help topics to Rust docs.
///
/// Just a cleaner abstraction over some very tedious/messy `#[doc]`
/// attributes. The goal of this is to use the API help topics as the
/// single source of truth for API help, instead of having to update
/// the docs in multiple places.
///
macro_rules! topics {
    (
        $(use HelpTopic $topic:ident for $desc:literal;)+
    ) => {
        pub mod topics {
            //! # Userscript API Help Topics
            //!
            //! This is a list of API help topics accessible from the
            //! userscript environment. To access a help topic in Lua,
            //! call:
            //!
            //! ```lua
            //! help 'topic_name'
            //! ```
            //!
            //! To list all help topics from Lua, call:
            //!
            //! ```lua
            //! help:topics()
            //! ```

            $(
                #[doc = concat!(
                    "# ",
                    $desc,
                    "\n\nTo access this help from Lua, call `help '",
                    stringify!($topic),
                    "'`.\n\n```txt\n",
                    include_str!(
                        concat!("help_system/topics/", stringify!($topic), ".txt")
                ))]
                pub mod $topic {
                    use crate::userscript_api::help_system::HelpTopic;

                    #[doc = "Userscript API help topic definition."]
                    pub struct Topic;

                    impl HelpTopic for Topic {
                        fn name(&self) -> &'static str {
                            stringify!($topic)
                        }

                        fn short_description(&self) -> &'static str {
                            $desc
                        }

                        fn content(&self) -> &'static str {
                            include_str!(
                                concat!("help_system/topics/", stringify!($topic), ".txt")
                            )
                        }
                    }
                }
            )+
        }
    };
}
pub(crate) use topics;
