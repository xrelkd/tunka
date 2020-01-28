#[macro_use]
extern crate serde;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate log;

use structopt::StructOpt;

mod command;
mod config;
mod context;
mod error;
mod tunnel;

fn main() {
    use crate::command::Command;
    let _ = simple_logger::init_with_level(log::Level::Info);

    match Command::from_args().run() {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            error!("{}", err);
            std::process::exit(1);
        }
    }
}
