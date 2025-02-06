//! # Helper macro definitions for sscan.
//!
//! This module defines various macros as convienience functions, both
//! for internal library usage and for external usage.
//!

/// # Automates the creation of the help system and registering topics.
///
/// This macro automates the creation and implementation of the
/// [`HelpSystem`] userscript API, which provides interactive help for
/// users within sscan. It performs the following tasks:
///
/// 1. Auto-generates the [`HelpSystem`] userscript API,
/// 2. Implements the required [`LuaUserData`] and [`ApiObject`] traits,
/// 3. Auto-generates [`HelpTopics`] from a module name, short
///    description, and a .TXT file under the `topics/` directory, for
///    every help topic specified when invoking this macro,
/// 4. Implements [`Default`] for [`HelpSystem`], which configues it
///    to autoload all help topics specified when invoking this macro,
/// 5. Auto-generates Rust modules for every help topic specified when
///    invoking this macro (see [`topics`] for generated modules),
/// 6. Auto-generates Rust documentation pages for every help topic specified
///    when invoking this macro.
///
/// ## Usage
///
/// This macro is intended for use only within
/// [`crate::userscript_api::help_system`]. For external crates, please
/// implement the [`HelpTopic`] trait to add help topics.
///
/// ```ignore
/// topics! {
///     use HelpTopic my_topic_1 for "Short description of topic 1.";
///     use HelpTopic my_topic_2 for "Short description of topic 2.";
///     ...
/// }
/// ```
///
/// The above code snippet would auto-generate rust modules `my_topic_1`
/// and `my_topic_2`, and create new [`HelpTopics`] by the same name,
/// using the short description and the file at `topics/my_topic_*.txt`.
///
/// [`HelpSystem`]: crate::userscript_api::help_system::HelpSystem
/// [`LuaUserData`]: crate::userscript_api::include::LuaUserData
/// [`ApiObject`]: crate::userscript_api::ApiObject
/// [`HelpTopics`]: crate::userscript_api::help_system::HelpTopic
/// [`HelpTopic`]: crate::userscript_api::help_system::HelpTopic
/// [`topics`]: crate::userscript_api::help_system::topics
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

        /// # The Userscript Help System API
        ///
        /// The Help System API exposes a function `help 'topic'` to the Lua
        /// userscript environment, which can print help on various topics to
        /// stdout. It is meant for interactive use, but can also be called from
        /// userscripts.
        ///
        /// Topics can be registered with [`HelpSystem::topic()`]. To create a
        /// new custom help topic, see [`HelpTopic`].
        pub struct HelpSystem {
            /// Holds the list of topics keyed by name.
            topics: HashMap<String, Box<dyn HelpTopic>>,
        }

        impl HelpSystem {
            /// Creates a new Help System instance with no topics loaded.
            #[must_use]
            pub fn new() -> Self {
                Self {
                    topics: HashMap::with_capacity(50),
                }
            }

            /// Registers a new [`HelpTopic`] with the Help System.
            pub fn topic(&mut self, topic: Box<dyn HelpTopic>) -> &mut Self {
                self.topics.insert(topic.name().to_owned(), topic);
                self
            }
        }

        impl UserData for HelpSystem {
            fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
                // Print generic help, or specific help if `topic` is specified.
                methods.add_meta_method("__call", |_, this: &HelpSystem, topic: Option<String>| {
                    if let Some(topic) = topic {
                        if let Some(topic) = this.topics.get(topic.trim()) {
                            let content: &str = topic.content();
                            println!("{content}");
                            if !content.ends_with('\n') {
                                println!();
                            }
                            Ok(())
                        } else {
                            Err(Error::topic_not_found(&topic).into_lua_err())
                        }
                    } else {
                        println!(include_str!("help_system/topics/__generic.txt"));
                        Ok(())
                    }
                });

                // List all available topics
                methods.add_method("topics", |_, this: &HelpSystem, ()| {
                    println!("The following help topics are available:\n");
                    for (name, topic) in &this.topics {
                        let name: &str = name.trim();
                        let description: &str = topic.short_description().trim();
                        println!("{name:<16} - {description:<50}");
                    }
                    println!("\nTo get help on a particular topic, use help 'topic'\n");
                    Ok(())
                });
            }
        }

        impl ApiObject for HelpSystem {
            fn name(&self) -> &'static str {
                "help"
            }
        }

        /// Registers all built-in help topics with the new [`HelpSystem`].
        impl Default for HelpSystem {
            fn default() -> Self {
                let mut help_system: HelpSystem = Self::new();
                $(
                    help_system.topic(Box::new($topic::Topic));
                )+
                help_system
            }
        }
    };
}
pub(crate) use topics;

/// Helper macro to impl Ping on many actors.
///
/// ## Usage
///
/// ```ignore
/// impl_ping!(Actor1, Actor2, Actor3, ...)
/// ```
macro_rules! impl_ping {
    ($($actor:ident),+) => {
        use kameo::message::{Context, Message};
        $(
            impl Message<Ping> for $actor {
                type Reply = ();
                async fn handle(&mut self, _: Ping, _: Context<'_, Self, Self::Reply>) -> Self::Reply {}
            }
        )+
    };
}
pub(crate) use impl_ping;
