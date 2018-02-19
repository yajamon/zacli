extern crate clap;
use clap::{App, Arg, SubCommand};

pub mod currencies;

pub trait Define {
    fn define<'a, 'b>() -> App<'a, 'b>;
}

pub fn define_subcommands<'a, 'b>(cmd: App<'a, 'b>) -> App<'a, 'b> {
    cmd.subcommand(currencies::Command::define()).subcommand(
        SubCommand::with_name("depth")
            .about("板情報を取得します")
            .arg(Arg::with_name("CURRENCY_PAIR"))
            .help("取引通貨の組み合わせ"),
    )
}
