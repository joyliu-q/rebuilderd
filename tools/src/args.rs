use rebuilderd_common::{Distro, Status};
use rebuilderd_common::errors::*;
use std::io::stdout;
use structopt::StructOpt;
use structopt::clap::{AppSettings, Shell};

#[derive(Debug, StructOpt)]
#[structopt(global_settings = &[AppSettings::ColoredHelp])]
pub struct Args {
    #[structopt(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    Status,
    Pkgs(Pkgs),
    Queue(Queue),
    /// Generate shell completions
    #[structopt(name="completions")]
    Completions(Completions),
}

#[derive(Debug, StructOpt)]
pub enum Pkgs {
    Sync(PkgsSync),
    Ls(PkgsList),
    SyncProfile(PkgsSyncProfile),
}

#[derive(Debug, StructOpt)]
pub struct PkgsSyncProfile {
    #[structopt(long="print-json")]
    pub print_json: bool,
    pub profile: String,
    #[structopt(long="sync-config", default_value="/etc/rebuilderd-sync.conf")]
    pub config_file: String,
}

#[derive(Debug, StructOpt)]
pub struct PkgsSync {
    #[structopt(long="print-json")]
    pub print_json: bool,
    #[structopt(long)]
    pub maintainer: Vec<String>,
    pub distro: Distro,
    pub suite: String,
    pub architecture: String,
    pub source: String,
}

#[derive(Debug, StructOpt)]
pub struct PkgsList {
    #[structopt(long)]
    pub name: Option<String>,
    #[structopt(long, possible_values=&["GOOD", "BAD", "UNKWN"])]
    pub status: Option<Status>,
    #[structopt(long)]
    pub distro: Option<String>,
    #[structopt(long)]
    pub suite: Option<String>,
    #[structopt(long)]
    pub architecture: Option<String>,
}

#[derive(Debug, StructOpt)]
pub enum Queue {
    Ls(QueueList),
    Push(QueuePush),
    #[structopt(name="drop")]
    Delete(QueueDrop),
}

#[derive(Debug, StructOpt)]
pub struct QueueList {
    #[structopt(long)]
    pub head: bool,
}

#[derive(Debug, StructOpt)]
pub struct QueuePush {
    pub distro: String,
    pub suite: String,
    #[structopt(long)]
    pub architecture: Option<String>,

    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug, StructOpt)]
pub struct QueueDrop {
    pub distro: String,
    pub suite: String,
    #[structopt(long)]
    pub architecture: Option<String>,

    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug, StructOpt)]
pub struct Completions {
    #[structopt(possible_values=&Shell::variants())]
    pub shell: Shell,
}

pub fn gen_completions(args: &Completions) -> Result<()> {
    Args::clap().gen_completions_to("rebuildctl", args.shell, &mut stdout());
    Ok(())
}