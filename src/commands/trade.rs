extern crate clap;
extern crate zaif_api;

use clap::{App, Arg, ArgMatches, SubCommand};
use self::zaif_api::AccessKey;
use self::zaif_api::trade_api::{TradeAction, TradeBuilder};

use commands::{Define, Run};
use config;

pub const COMMAND_NAME: &str = "trade";
pub struct Command;

const ARG_CURRENCY_PAIR: &str = "CURRENCY_PAIR";
const ARG_ACTION: &str = "ACTION";
const ARG_AMOUNT: &str = "AMOUNT";
const ARG_PRICE: &str = "PRICE";
const ARG_LIMIT: &str = "LIMIT";

impl Define for Command {
    fn define<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(COMMAND_NAME)
            .about("トレードします")
            .arg(
                Arg::with_name(ARG_CURRENCY_PAIR)
                    .required(true)
                    .help("取引する通貨ペア"),
            )
            .arg(
                Arg::with_name(ARG_ACTION)
                    .required(true)
                    .possible_values(&["ask", "bid"])
                    .help("取引の種類 ask:売り注文 bid:買い注文"),
            )
            .arg(
                Arg::with_name(ARG_PRICE)
                    .required(true)
                    .takes_value(true)
                    .help("取引の単価"),
            )
            .arg(
                Arg::with_name(ARG_AMOUNT)
                    .required(true)
                    .takes_value(true)
                    .help("取引する数量"),
            )
            .arg(
                Arg::with_name(ARG_LIMIT)
                    .long("limit")
                    .takes_value(true)
                    .help("リミット単価"),
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

        let action = match matches.value_of(ARG_ACTION).unwrap() {
            "ask" => TradeAction::Ask,
            "bid" => TradeAction::Bid,
            _ => TradeAction::None,
        };

        let api = &mut TradeBuilder::new();
        api.access_key(AccessKey::new(&config.access_key, &config.access_secret))
            .currency_pair(matches.value_of(ARG_CURRENCY_PAIR).unwrap().to_string())
            .action(action)
            .amount(matches.value_of(ARG_AMOUNT).unwrap().parse().unwrap())
            .price(matches.value_of(ARG_PRICE).unwrap().parse().unwrap());

        if let Some(limit) = matches.value_of(ARG_LIMIT) {
            api.limit(Some(limit.parse().unwrap()));
        }

        let result = api.finalize().exec().unwrap();
        println!(
            "約定数: {}, 板登録数: {}",
            result.received, result.remains
        );
    }
}
