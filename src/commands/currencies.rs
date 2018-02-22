extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
extern crate zaif_api;
use self::zaif_api::public_api::*;

use commands::{Define, Run};

pub const COMMAND_NAME: &str = "currencies";
pub struct Command;

impl Define for Command {
    fn define<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(COMMAND_NAME)
            .about("通貨情報を取得します")
            .arg(Arg::with_name("NAME").help("通貨の名前"))
    }
}

impl Run for Command {
    fn name<'a>() -> &'a str {
        COMMAND_NAME
    }

    fn run<'a>(matches: &ArgMatches<'a>) {
        let api = CurrenciesBuilder::new()
            .name(matches.value_of("NAME").unwrap_or("all").to_string())
            .finalize();
        for currency in api.exec().unwrap() {
            println!("{}", currency.name);
        }
    }
}
