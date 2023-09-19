use std::path::PathBuf;
use clap::Parser;

//------------------------------------------------------------------------------

/// MQTT to Statsd bridge.
#[derive(Parser, Debug)]
#[clap( name = "mqtt2statsd", version = clap::crate_version!())]
pub struct Args {
    /// Log MQTT & statsd messages on the console
    #[arg(short, long)]
    pub verbose: bool,

    /// Configuration file
    pub configuration: PathBuf
}

//------------------------------------------------------------------------------

pub fn parse() -> Args {
    Args::parse()
}
