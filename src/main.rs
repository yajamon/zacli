#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    app_from_crate!().get_matches();
}
