#[macro_use]
extern crate clap;
extern crate toml;

mod error;
mod commands;
mod config;

pub use error::{Error, Result};

fn main() {
    let app = commands::define_subcommands(app_from_crate!());

    let matches = app.get_matches();

    commands::dispatch(matches);
}
