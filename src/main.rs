mod options;
mod data;

use crate::options::{Options, Parser, SubCommand};

fn main() -> Result<(), std::io::Error> {
    let options = Options::parse();

    match options.sub_command {
        Some(SubCommand::Init) => init(),
        None => init(),
    }
}

fn init() -> Result<(), std::io::Error> {
    data::init()
   
}
