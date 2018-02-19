extern crate clap;
use clap::{App, Arg, SubCommand};

use commands::Define;

pub struct Command;

impl Define for Command {
    fn define<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("currencies")
            .about("通貨情報を取得します")
            .arg(Arg::with_name("NAME").help("通貨の名前"))
    }
}
