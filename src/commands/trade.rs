extern crate clap;
extern crate zaif_api;

use clap::{App, Arg, ArgMatches, SubCommand};
use self::zaif_api::trade_api::TradeBuilder;

use commands::{Define, Run};
use config;

pub const COMMAND_NAME: &str = "trade";
pub struct Command;

impl Define for Command {
    fn define<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(COMMAND_NAME)
            .about("トレードします")
            .arg(Arg::with_name("Argument_name").help("command help"))
    }
}

impl Run for Command {
    fn name<'a>() -> &'a str {
        COMMAND_NAME
    }

    fn run<'a>(matches: &ArgMatches<'a>) {
        let file_path = config::default_path().unwrap();
        let config = config::open_config(file_path.as_path()).unwrap();

        let api = TradeBuilder::new().finalize();
        let result = api.exec().unwrap();
        println!("{}", "result...");
    }
}
