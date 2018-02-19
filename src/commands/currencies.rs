extern crate clap;
use clap::{App, Arg, SubCommand};

use commands::Command;

pub struct Currencies;

impl Command for Currencies {
    fn define<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("currencies")
            .about("通貨情報を取得します")
            .arg(Arg::with_name("NAME").help("通貨の名前"))
    }
}
