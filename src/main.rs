extern crate clap;
use clap::App;

fn main() {
    App::new("zacli")
        .version("0.1.0")
        .about("Zaif api client for command line interface")
        .author("yajamon <yajamon.tatsuki@gmail.com>")
        .get_matches();
}
