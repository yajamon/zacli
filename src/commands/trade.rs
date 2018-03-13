extern crate clap;
extern crate zaif_api;

use clap::{App, Arg, ArgMatches, SubCommand};
use self::zaif_api::AccessKey;
use self::zaif_api::trade_api::TradeBuilder;

use commands::{Define, Run};
use config;

pub const COMMAND_NAME: &str = "trade";
pub struct Command;

const ARG_CURRENCY_PAIR: &str = "CURRENCY_PAIR";

impl Define for Command {
    fn define<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(COMMAND_NAME)
            .about("トレードします")
            .arg(
                Arg::with_name(ARG_CURRENCY_PAIR)
                    .required(true)
                    .help("取引する通貨ペア"),
            )
    }
}

impl Run for Command {
    fn name<'a>() -> &'a str {
        COMMAND_NAME
    }

    fn run<'a>(matches: &ArgMatches<'a>) {
        let file_path = config::default_path().unwrap();
        let config = config::open_config(file_path.as_path()).unwrap();

        let mut api = TradeBuilder::new()
            .access_key(AccessKey::new(&config.access_key, &config.access_secret))
            .currency_pair(matches.value_of(ARG_CURRENCY_PAIR).unwrap().to_string())
            .finalize();

        let result = api.exec().unwrap();
        println!("{}", "result...");
    }
}
