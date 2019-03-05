use clap::{Arg, App};

const PROGRAM_NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let matches = App::new(PROGRAM_NAME)
                    .version(VERSION)
                    .author(AUTHOR)
                    .about(DESCRIPTION)
                    .arg(Arg::with_name("FILE")
                        .help("Sets the input file to convert")
                        .short("f")
                        .long("file")
                        .value_name("FILE")
                        .takes_value(true))
                    .arg(Arg::with_name("OUTPUT")
                        .help("Sets the output directory to store output files")
                        .short("o")
                        .long("output")
                        .required(false)
                        .value_name("OUTPUT")
                        .takes_value(true))
                    .get_matches();

    println!("commands!");
}
