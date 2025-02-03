//! # Retrieve Program Information
//!
//! The [`AboutApi`] provides userscript access to build and version
//! information, as well as licensing and contributions info. The
//! module also writes global constants for quick access to this
//! information.
//!
//! ## Userscript API
//!
//! This is a userscript API. The API's functionality is registered with
//! the Lua virtual machine, where userscripts can call into it.
//!

use crate::userscript_api::{ApiObject, include::{LuaUserData, LuaUserDataMethods, LuaUserDataRef, LuaTable}};

/// Extended attribution information
const LICENSE_EXT: &str = "\
sscan is made possible thanks to the use of open-source software. A full
list of software, the authors, and package licenses can be found in the
OPEN_SOURCE_LICENSES.md file of the source repository.
";

/// # The Program Information and License API
///
/// The information APIs and global constants expose build, versioning,
/// and license information to the userscript environment.
pub struct AboutApi {
    pkg_name: String,
    pkg_description: String,
    pkg_authors: String,
    pkg_license: String,
    docs_link: String,
    license: String,
    powered_by: String,
    repo: String,
    version_major: u16,
    version_minor: u16,
    version_patch: u16,
}

impl Default for AboutApi {
    fn default() -> Self {
        let pkg_authors: String = env!("CARGO_PKG_AUTHORS").to_owned();
        let pkg_description: String = env!("CARGO_PKG_DESCRIPTION").to_owned();
        let pkg_license: String = env!("CARGO_PKG_LICENSE").to_owned();
        let pkg_name: String = env!("CARGO_PKG_NAME").to_owned();
        let repo: String = env!("CARGO_PKG_REPOSITORY").to_owned();
        let docs_link: String = format!("https://docs.rs/{pkg_name}/latest/{pkg_name}");
        let license: String = include_str!("../../LICENSE.md").to_owned();
        let license: String = format!("{license}\n{LICENSE_EXT}\n{repo}");
        let powered_by: String = format!(
            "{} ({})\nSource: {}",
            "Lua 5.4",
            "Copyright (c) 1994–2024 Lua.org, PUC-Rio.",
            "https://lua.org"
        );
        let version_major: u16 = env!("CARGO_PKG_VERSION_MAJOR").parse::<u16>().unwrap();
        let version_minor: u16 = env!("CARGO_PKG_VERSION_MINOR").parse::<u16>().unwrap();
        let version_patch: u16 = env!("CARGO_PKG_VERSION_PATCH").parse::<u16>().unwrap();

        Self { pkg_name, pkg_description, pkg_authors, pkg_license, docs_link, license, powered_by, repo, version_major, version_minor, version_patch }
    }
}

impl LuaUserData for AboutApi {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("docs", |_, this: &AboutApi| {
            Ok(this.docs_link.clone())
        });

        fields.add_field_method_get("license", |_, this: &AboutApi| {
            Ok(this.license.clone())
        });

        fields.add_field_method_get("program", |_, this: &AboutApi| {
            Ok(this.pkg_name.clone())
        });

        fields.add_field_method_get("lua", |_, this: &AboutApi| {
            Ok(this.powered_by.clone())
        });

        fields.add_field_method_get("repo", |_, this: &AboutApi| {
            Ok(this.repo.clone())
        });

        fields.add_field_method_get("version", |_, this: &AboutApi| {
            Ok(format!("{}.{}.{}", this.version_major, this.version_minor, this.version_patch))
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_meta_method("__call", |_, this: LuaUserDataRef<AboutApi>, ()| async move {
            let about_info: String = format!(
                "{} v{}.{}.{} - {}\n\nAuthors: {}\nRepository: {}\nDocs: {}\nLicense: {}\n",
                this.pkg_name,
                this.version_major,
                this.version_minor,
                this.version_patch,
                this.pkg_description,
                this.pkg_authors,
                this.repo,
                this.docs_link,
                this.pkg_license,
            );
            Ok(about_info)
        });
    }
}

impl ApiObject for AboutApi {
    fn name(&self) -> &'static str {
        "about"
    }

    fn init_script(&self, lua: &mlua::Lua) -> mlua::Result<()> {
        let globals: LuaTable = lua.globals();

        // Set global info variables
        globals.set("_VERSION", format!("{} v{}.{}.{}", self.pkg_name, self.version_major, self.version_minor, self.version_patch))?;
        globals.set("_LICENSE", self.license.as_str())?;
        globals.set("_POWERED_BY", self.powered_by.as_str())?;
        globals.set("_DOCS", format!(
            "{}\n\n  {}\n\n{}\n\n  {}\n{}\n{}\n\n",
            "To view online help, see:",
            self.docs_link,
            "Or, to access built-in help, call:",
            "help()            -- View general help information",
            "help:topics()     -- List available help topics",
            "help 'topic_name' -- View detailed help on `topic_name`"
        ))?;

        // Set the build info table
        let build_info: LuaTable = lua.create_table()?;
        build_info.set("major", self.version_major)?;
        build_info.set("minor", self.version_minor)?;
        build_info.set("patch", self.version_patch)?;
        globals.set("_BUILD", build_info)?;
        Ok(())
    }
}
