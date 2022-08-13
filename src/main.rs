mod options;
mod data;
mod error;

use data::hash_file;
use data::cat_file;

use crate::options::{Options, Parser, SubCommand};
use crate::error::Result;

fn main() -> Result<()> {
    let options = Options::parse();

    match options.sub_command {
        Some(SubCommand::Init) => {init()?;}
        Some(SubCommand::HashObject(hashobject)) => {hash_file(hashobject.file_name)?;}
        Some(SubCommand::CatFile(catfile)) => {cat_file(catfile.oid)?;}
        None => {init()?;}
    };
    Ok(())
}

fn init() -> Result<()> {
    data::init()
}
