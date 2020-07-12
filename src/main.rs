#[macro_use]
extern crate serde;

#[macro_use]
extern crate log;

mod command;
mod config;
mod context;
mod error;
mod tunnel;

use self::{command::Command, error::Error};

fn main() -> Result<(), Error> {
    let _ = simple_logger::init_with_level(log::Level::Info);
    let cmd = Command::new();
    cmd.run()
}
