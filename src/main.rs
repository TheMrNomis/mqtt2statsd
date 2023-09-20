use std::{thread, error::Error, time::Duration, collections::HashMap};

use log::{error, warn, info, debug};
use simplelog::{ TermLogger, LevelFilter, ConfigBuilder, TerminalMode, ColorChoice};
use paho_mqtt as paho;

use statsd;

mod cli_args;
mod config;
mod mqtt;

//------------------------------------------------------------------------------

fn try_reconnect( mqtt: &paho::Client, timeout: u32 ) -> bool {
    warn!("MQTT server lost, trying to reconnect...");
    for _ in 0..timeout {
        thread::sleep(Duration::from_secs(1));
        if mqtt.reconnect().is_ok() {
            info!("MQTT server reconnected");
            return true;
        }
    }

    error!("MQTT server timed out");
    false
}

//------------------------------------------------------------------------------

fn handle_msg(
    msg:    &paho::Message,
    topics: &HashMap<String, String>,
    statsd: &statsd::Client ) {

    let value = msg.payload_str().parse::<f64>();
    if value.is_err() {
        warn!(
            "{}: message \"{}\" is not a number",
            msg.topic(),
            msg.payload_str()
        );
        return;
    }
    let value = value.unwrap();

    let mqtt_topic = msg.topic();
    let statsd_topic = topics.get(mqtt_topic).unwrap();

    statsd.gauge(statsd_topic, value);
    info!("{} ({}): {}", mqtt_topic, statsd_topic, value);
}

//------------------------------------------------------------------------------

fn main_impl( args: &cli_args::Args ) -> Result<(), Box<dyn Error>> {
    let config = config::from_file(&args.configuration)?;
    debug!("Config: {:?}", config);

    // init MQTT
    let mqtt = mqtt::create( &config.mqtt )?;
    let rx = mqtt.start_consuming();
    mqtt::subscribe( &mqtt, &config.topics, args.verbose );

    // init statsd
    let statsd = statsd::Client::new(
        format!("{}:{}", config.statsd.hostname, config.statsd.port),
        &config.statsd.prefix
    )?;

    // MQTT -> statsd topic translation table
    let mut topics = HashMap::new();
    for topic in config.topics {
        topics.insert( topic.mqtt, topic.statsd );
    }
    let topics = topics;

    // handle ^C signal to quit gracefully
    let ctrlc_mqtt = mqtt.clone();
    ctrlc::set_handler(move || {
        ctrlc_mqtt.stop_consuming()
    })?;

    // main event loop
    for msg in rx.iter() {
        if !mqtt.is_connected() && !try_reconnect(&mqtt, config.mqtt.timeout) {
            break;
        }

        if let Some(msg) = msg {
            handle_msg(&msg, &topics, &statsd);
        }
    }

    // clean up before quitting
    if mqtt.is_connected() {
        info!("Disconnecting");
        mqtt.disconnect(None)?;
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
