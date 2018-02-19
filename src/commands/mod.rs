extern crate clap;
use clap::{App, Arg, SubCommand};

pub mod currencies;
pub mod depth;

pub trait Define {
    fn define<'a, 'b>() -> App<'a, 'b>;
}

pub fn define_subcommands<'a, 'b>(cmd: App<'a, 'b>) -> App<'a, 'b> {
    cmd.subcommand(currencies::Command::define())
        .subcommand(depth::Command::define())
}
