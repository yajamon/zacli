extern crate clap;
use clap::{App, ArgMatches};

pub mod currencies;
pub mod currency_pairs;
pub mod depth;
pub mod config;

pub trait Define {
    fn define<'a, 'b>() -> App<'a, 'b>;
}

pub fn define_subcommands<'a, 'b>(cmd: App<'a, 'b>) -> App<'a, 'b> {
    cmd.subcommand(config::Command::define())
        .subcommand(currencies::Command::define())
        .subcommand(currency_pairs::Command::define())
        .subcommand(depth::Command::define())
}

pub trait Run {
    fn name<'a>() -> &'a str;
    fn run<'a>(matches: &ArgMatches<'a>);
}

pub fn dispatch<'a>(matches: ArgMatches<'a>) {
    match matches.subcommand() {
        (config::COMMAND_NAME, Some(sub_m)) => config::Command::run(sub_m),
        (currencies::COMMAND_NAME, Some(sub_m)) => currencies::Command::run(sub_m),
        (currency_pairs::COMMAND_NAME, Some(sub_m)) => currency_pairs::Command::run(sub_m),
        (depth::COMMAND_NAME, Some(sub_m)) => depth::Command::run(sub_m),
        _ => {}
    }
}
