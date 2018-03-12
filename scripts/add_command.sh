#!/bin/bash

readonly ROOT_DIR=$(cd $(dirname $(dirname $BASH_SOURCE)); pwd)
readonly COMMANDS_DIR="$ROOT_DIR/src/commands"

readonly COMMAND_NAME=$1
if [ -z $COMMAND_NAME ]; then
    echo 'Require command name' >&2
    exit 1
fi

readonly COMMAND_NAME_SNAKE=$(echo $COMMAND_NAME | perl -e '$_ = <STDIN>; s/^([A-Z])/\L\1\E/; s/([A-Z])/_\L\1\E/g; print $_')
readonly COMMAND_NAME_PASCAL=$(echo $COMMAND_NAME_SNAKE | perl -e '$_ = <STDIN>; s/(^|_)(.)/\U\2\E/g; print $_')

cat << EOT
extern crate clap;
extern crate zaif_api;

use clap::{App, Arg, ArgMatches, SubCommand};
use self::zaif_api::public_api::${COMMAND_NAME_PASCAL}Builder;

use commands::{Define, Run};

pub const COMMAND_NAME: &str = "${COMMAND_NAME_SNAKE}";
pub struct Command;

impl Define for Command {
    fn define<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(COMMAND_NAME)
            .about("command description")
            .arg(Arg::with_name("Argument_name").help("command help"))
    }
}

impl Run for Command {
    fn name<'a>() -> &'a str {
        COMMAND_NAME
    }

    fn run<'a>(matches: &ArgMatches<'a>) {
        let api = ${COMMAND_NAME_PASCAL}Builder::new()
            .finalize();
        let result = api.exec().unwrap();
        println!("{}", "result...");
    }
}
EOT
