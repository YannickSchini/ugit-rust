mod options;

use crate::options::{Options, Parser, SubCommand};

fn main() {
    let options = Options::parse();

    match options.sub_command {
        Some(SubCommand::Init) => init(),
        None => init(),
    }
}

fn init() {
    println!("Hello world")
}
