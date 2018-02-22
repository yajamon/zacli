extern crate clap;
extern crate zaif_api;

use clap::{App, Arg, ArgMatches, SubCommand};
use self::zaif_api::public_api::DepthBuilder;

use commands::{Define, Run};

pub const COMMAND_NAME: &str = "depth";
pub struct Command;

impl Define for Command {
    fn define<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(COMMAND_NAME)
            .about("板情報を取得します")
            .arg(Arg::with_name("CURRENCY_PAIR"))
            .help("取引通貨の組み合わせ")
    }
}

impl Run for Command {
    fn name<'a>() -> &'a str {
        COMMAND_NAME
    }

    fn run<'a>(matches: &ArgMatches<'a>) {
        let api = DepthBuilder::new()
            .currency_pair(matches.value_of("CURRENCY_PAIR").unwrap().to_string())
            .finalize();
        for ask in api.exec().unwrap().asks {
            println!("ask price: {}, amount: {}", ask.price(), ask.amount());
        }
    }
}
