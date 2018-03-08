extern crate clap;

use std::env;
use clap::{App, Arg, ArgMatches, SubCommand};

use commands::{Define, Run};
use config;

pub const COMMAND_NAME: &str = "config";
pub struct Command;

const ARG_INITIALISE: &str = "initialize";

impl Define for Command {
    fn define<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(COMMAND_NAME)
            .about("設定を閲覧したり、編集できます。")
            .arg(
                Arg::with_name(ARG_INITIALISE)
                    .long("init")
                    .help("設定ファイルをホームディレクトリに作成します。"),
            )
    }
}

impl Run for Command {
    fn name<'a>() -> &'a str {
        COMMAND_NAME
    }

    fn run<'a>(matches: &ArgMatches<'a>) {
        let file_path = config::default_path().unwrap();
        if matches.is_present(ARG_INITIALISE) {
            let c = config::new();
            config::save_config(file_path.as_path(), &c).unwrap();
            println!(
                "設定ファイルを作成しました: {}",
                file_path.to_str().unwrap()
            );
            println!("設定ファイルの権限を確認してください。");
            return;
        }
        let config = config::open_config(file_path.as_path()).unwrap();
    }
}
