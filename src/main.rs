#[macro_use]
extern crate clap;
extern crate zaif_api;

use zaif_api::public_api::*;

mod commands;

fn main() {
    let app = commands::define_subcommands(app_from_crate!());

    let matches = app.get_matches();

    if let Some(ref matches) = matches.subcommand_matches("currencies") {
        let api = CurrenciesBuilder::new()
            .name(matches.value_of("NAME").unwrap_or("all").to_string())
            .finalize();
        for currency in api.exec().unwrap() {
            println!("{}", currency.name);
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("depth") {
        let api = DepthBuilder::new()
            .currency_pair(matches.value_of("CURRENCY_PAIR").unwrap().to_string())
            .finalize();
        for ask in api.exec().unwrap().asks {
            println!("ask price: {}, amount: {}", ask.price(), ask.amount());
        }
    }
}
