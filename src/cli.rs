use std::path::PathBuf;

use clap::{Args, Parser};
use secrecy::SecretString;

#[derive(Parser, Debug)]
#[command(version)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(flatten)]
    pub options: GlobalOptions,

    #[command(subcommand)]
    pub command: Subcommand,
}

#[derive(Args, Debug)]
pub struct GlobalOptions {
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,
}

#[derive(clap::Subcommand, Debug)]
pub enum Subcommand {
    Sync(SyncOptions),
    // Watch {},
}

#[derive(Args, Debug)]
pub struct SyncOptions {
    /// (Roblox targets only) Open Cloud API key.
    #[arg(short, long, env = "RUNWAY_API_KEY")]
    pub api_key: Option<SecretString>,

    #[command(flatten)]
    pub creator: Option<Creator>,

    /// Path to config file or directory containing config file.
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Which target to sync to.
    #[arg(short, long)]
    pub target: String,

    /// Ignore previous state and resync everything.
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct Creator {
    /// (Roblox targets only) Sync to a user
    #[arg(short, long, group = "creator", env = "RUNWAY_USER_ID")]
    pub user_id: Option<String>,

    /// (Roblox targets only) Sync to a group
    #[arg(short, long, group = "creator", env = "RUNWAY_GROUP_ID")]
    pub group_id: Option<String>,
}
