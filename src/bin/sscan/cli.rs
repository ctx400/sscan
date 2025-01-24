use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(infer_subcommands = true)]
pub struct CliArgs {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Run sscan with the specified userscript.
    Run {
        /// Path to the userscript sscan should run.
        script: PathBuf,
    },

    /// Start sscan in interactive mode.
    Interactive {
        /// If specified, runs a userscript before launching the REPL.
        #[arg(short, long)]
        startup_script: Option<PathBuf>,

        /// If set, silences the spash message on REPL startup.
        #[arg(short, long)]
        nosplash: bool,
    },
}
