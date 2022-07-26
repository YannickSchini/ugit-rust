use std::path::PathBuf;

pub use clap::Parser;
use clap::{Subcommand, Args};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Options {
    #[clap(subcommand)]
    pub sub_command: Option<SubCommand>,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Init,
    HashObject(HashObject),
}

#[derive(Debug, Args)]
pub struct HashObject {
    pub file_name: PathBuf,
}
