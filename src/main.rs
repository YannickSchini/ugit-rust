use clap::Parser;

#[derive(Parser, Debug, PartialEq)]
struct Args {
    #[clap(short, long, value_parser, default_value = "init")]
    subcommand: String
}


fn main() {
    let args = Args::parse();

    if args.subcommand == "init" {
        println!("Hello, world!");
    }
}
