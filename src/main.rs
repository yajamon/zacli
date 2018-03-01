#[macro_use]
extern crate clap;

mod commands;
mod config;

fn main() {
    let app = commands::define_subcommands(app_from_crate!());

    let matches = app.get_matches();

    commands::dispatch(matches);
}
