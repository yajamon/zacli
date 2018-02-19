extern crate clap;

use clap::App;

pub mod currencies;

pub trait Command {
    fn define<'a, 'b>() -> App<'a, 'b>;
}
