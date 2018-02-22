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

pub trait Run {
    fn name<'a>() -> &'a str;
    fn run<'a>(matches: &ArgMatches<'a>);
}

pub fn dispatch<'a>(matches: ArgMatches<'a>) {
    match matches.subcommand() {
        (currencies::COMMAND_NAME, Some(sub_m)) => currencies::Command::run(sub_m),
        _ => {}
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
