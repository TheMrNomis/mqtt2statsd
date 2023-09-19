use std::error::Error;

extern crate log;
use log::{error, info, debug};

use simplelog::{ TermLogger, LevelFilter, ConfigBuilder, TerminalMode, ColorChoice};

use paho_mqtt::Message;

mod cli_args;
mod config;
mod mqtt;

fn handle_msg( msg: &Message )
{
    info!("New message: {}", msg)
}

//------------------------------------------------------------------------------

fn main_impl( args: &cli_args::Args ) -> Result<(), Box<dyn Error>> {
    let config = config::from_file(&args.configuration)?;
    debug!("Config: {:?}", config);

    // init MQTT
    let mqtt = mqtt::create( &config.mqtt )?;
    let rx = mqtt.start_consuming();
    mqtt::subscribe( &mqtt, &config.topics, args.verbose );

    // main event loop
    for msg in rx.iter() {
        if !mqtt.is_connected() {
            if args.verbose {
                println!( )
            }
            break;
        }

        if let Some(msg) = msg {
            handle_msg(&msg)
        }
    }

    if mqtt.is_connected() {
        println!("Disconnecting");
        //mqtt.unsubscribe_many(topics)
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn main() {
    let args = cli_args::parse();

    // set up logging
    TermLogger::init(
        match args.verbose {
            true  => LevelFilter::Debug,
            false => LevelFilter::Info
        },
        ConfigBuilder::new()
            .set_time_format_rfc3339()
            .set_time_offset_to_local().unwrap()
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto
    ).unwrap();

    // actually run the program
    if let Err(e) = main_impl(&args) {
        error!("Error: {}", e)
    }
}
