extern crate clap;
extern crate zaif_api;

use clap::{App, Arg, ArgMatches, SubCommand};
use self::zaif_api::public_api::{CurrencyPairsBuilder, CurrencyPairsResponse};

use commands::{Define, Run};

pub const COMMAND_NAME: &str = "currency_pairs";
pub struct Command;

impl Define for Command {
    fn define<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(COMMAND_NAME)
            .about("通貨ペア情報を取得します")
            .arg(
                Arg::with_name("CURRENCY_PAIR")
                    .help("通貨ペア 省略した場合、allが指定されたものとする"),
            )
    }
}

impl Run for Command {
    fn name<'a>() -> &'a str {
        COMMAND_NAME
    }

    fn run<'a>(matches: &ArgMatches<'a>) {
        let api = CurrencyPairsBuilder::new()
            .currency_pair(
                matches
                    .value_of("CURRENCY_PAIR")
                    .unwrap_or("all")
                    .to_string(),
            )
            .finalize();
        let result = api.exec().unwrap();
        full_display(result);
    }
}

fn full_display(currency_pairs: Vec<CurrencyPairsResponse>) {
    for pair in currency_pairs {
        let kind = match pair.is_token {
            true => "トークン",
            false => "主要通貨",
        };
        println!("currency_pair: {}", pair.currency_pair);
        println!("         名称: {}", pair.name);
        println!("         種別: {}", kind);
        println!(" 取引数量単位: {}", pair.item_unit_step);
        println!(" 取引金額単位: {}", pair.aux_unit_step);
    }
}
