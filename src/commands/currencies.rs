extern crate clap;
use clap::{App, Arg, SubCommand};

use commands::Define;

const COMMAND_NAME: &str = "currencies";
pub struct Command;

impl Define for Command {
    fn define<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(COMMAND_NAME)
            .about("通貨情報を取得します")
            .arg(Arg::with_name("NAME").help("通貨の名前"))
    }
}
