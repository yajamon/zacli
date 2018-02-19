extern crate clap;
use clap::{App, Arg, SubCommand};

use commands::Define;

pub struct Command;

impl Define for Command {
    fn define<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("depth")
            .about("板情報を取得します")
            .arg(Arg::with_name("CURRENCY_PAIR"))
            .help("取引通貨の組み合わせ")
    }
}
