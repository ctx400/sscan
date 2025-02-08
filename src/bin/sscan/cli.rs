use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(infer_subcommands = true)]
pub struct Args {
    /// Load unsafe Lua standard libraries.
    ///
    /// If set, sscan will load unsafe Lua standard libraries, such as
    /// the `debug` library, alongside the usual "safe" standard
    /// libraries.
    ///
    /// WARNING: Incorrect use of the Lua debug library and other unsafe
    /// libraries may cause undefined behavior, which can cause panics
    /// or other unpredictable side effects. Unsafe mode is intended
    /// only for advanced users, and for testing purposes only.
    ///
    /// Userscripts in production should never rely on unsafe
    /// functionality, as it introduces security and isolation risks.
    #[arg(short, long)]
    pub unsafe_mode: bool,

    /// The runtime action to take.
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Run sscan with the specified userscript.
    Run {
        /// Path to the userscript sscan should run.
        script: PathBuf,

        /// Arguments to pass to <SCRIPT>.
        ///
        /// All extra arguments are passed to Lua, and userscripts can
        /// access them through the global `arg` array.
        #[arg(allow_hyphen_values(true), allow_negative_numbers(true))]
        args: Vec<String>,
    },

    /// Start sscan in interactive mode.
    Interactive {
        /// If specified, runs a userscript before launching the REPL.
        #[arg(short, long)]
        startup_script: Option<PathBuf>,

        /// If set, silences the spash message on REPL startup.
        #[arg(short, long)]
        nosplash: bool,

        /// Arguments to pass to the interactive environment.
        ///
        /// All extra arguments are passed to Lua, and are accessible
        /// through the global `arg` array. If `--startup-script`
        /// is passed, the script will also have access to these args.
        #[arg(allow_hyphen_values(true), allow_negative_numbers(true))]
        args: Vec<String>,
    },
}
