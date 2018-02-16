#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .get_matches();
}
