extern crate clap;
extern crate zaif_api;
use clap::{App, Arg, ArgMatches};
use self::zaif_api::public_api::*;

pub mod currencies;
pub mod depth;

pub trait Define {
    fn define<'a, 'b>() -> App<'a, 'b>;
}

pub fn define_subcommands<'a, 'b>(cmd: App<'a, 'b>) -> App<'a, 'b> {
    cmd.subcommand(currencies::Command::define())
        .subcommand(depth::Command::define())
}

pub fn dispatch<'a>(matches: ArgMatches<'a>) {
    if let Some(ref matches) = matches.subcommand_matches("currencies") {
        let api = CurrenciesBuilder::new()
            .name(matches.value_of("NAME").unwrap_or("all").to_string())
            .finalize();
        for currency in api.exec().unwrap() {
            println!("{}", currency.name);
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("depth") {
        let api = DepthBuilder::new()
            .currency_pair(matches.value_of("CURRENCY_PAIR").unwrap().to_string())
            .finalize();
        for ask in api.exec().unwrap().asks {
            println!("ask price: {}, amount: {}", ask.price(), ask.amount());
        }
    }
}
