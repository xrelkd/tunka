mod command;
mod config;
mod context;
mod error;
mod tunnel;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use self::command::Cli;

fn init_tracing() {
    // filter
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // format
    let fmt_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry().with(filter_layer).with(fmt_layer).init();
}

fn main() {
    init_tracing();
    if let Err(err) = Cli::default().run() {
        eprintln!("{err}");
        std::process::exit(-1);
    }
}
