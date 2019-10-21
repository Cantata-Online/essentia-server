extern crate clap;
use clap::{Arg, App};

pub struct Arguments {
    pub config_file: String
}

impl Arguments {
    pub fn create() -> Arguments {
        Arguments{
            config_file: String::from("")
        }
    }
}

pub fn parse() -> Arguments {
    let matches = App::new("Essentia server")
        .version("0.0.1")
        .about("A server application for Essentia Online")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Path to config file")
            .takes_value(true))
        .get_matches();

    let mut args = Arguments::create();
    args.config_file = String::from(matches.value_of("config").unwrap_or("config.yaml"));

    return args;
}