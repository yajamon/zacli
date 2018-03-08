extern crate clap;

use std::env;
use clap::{App, Arg, ArgMatches, SubCommand};

use commands::{Define, Run};
use config;

pub const COMMAND_NAME: &str = "config";
pub struct Command;

impl Define for Command {
    fn define<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(COMMAND_NAME)
            .about("設定を閲覧したり、編集できます。")
    }
}

impl Run for Command {
    fn name<'a>() -> &'a str {
        COMMAND_NAME
    }

    fn run<'a>(matches: &ArgMatches<'a>) {
        let file_path = config::default_path().unwrap();
        let config = config::open_config(file_path.as_path()).unwrap();
        println!("version {}", config.version);
    }
}
