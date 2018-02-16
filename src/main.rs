#[macro_use]
extern crate clap;
extern crate zaif_api;

use clap::{Arg, SubCommand};
use zaif_api::public_api::*;

fn main() {
    let app = app_from_crate!().subcommand(
        SubCommand::with_name("currencies")
            .about("通貨情報を取得します")
            .arg(Arg::with_name("NAME").help("通貨の名前")),
    );

    let matches = app.get_matches();

    if let Some(ref matches) = matches.subcommand_matches("currencies") {
        let api = CurrenciesBuilder::new()
            .name(matches.value_of("NAME").unwrap_or("all").to_string())
            .finalize();
        for currency in api.exec().unwrap() {
            println!("{}", currency.name);
        }
    }
}
