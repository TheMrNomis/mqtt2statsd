use std::path::PathBuf;
use std::fs;

use clap::Parser;
use serde::Deserialize;

extern crate paho_mqtt as mqtt;

// =============================================================================
// CLI arguments
// =============================================================================

/// MQTT to Statsd bridge.
#[derive(Parser, Debug)]
#[clap( name = "mqtt2statsd", version = clap::crate_version!())]
struct Args {
    /// Log MQTT & statsd messages on the console
    #[arg(short, long)]
    verbose: bool,

    /// Configuration file
    configuration: PathBuf
}

// =============================================================================
// Configuration file structure
// =============================================================================

#[derive(Debug, Deserialize)]
struct Config {
    mqtt: MqttConfig,
    statsd: StatsdConfig,
    topics: Vec<Topic>
}

//------------------------------------------------------------------------------

fn default_mqtt_port() -> u32 { 1883 }

#[derive(Debug, Deserialize)]
struct MqttConfig {
    hostname: String,

    #[serde(default = "default_mqtt_port") ]
    port: u32
}

//------------------------------------------------------------------------------

fn default_statsd_port() -> u32 { 8125 }

#[derive(Debug, Deserialize)]
struct StatsdConfig {
    hostname: String,

    #[serde(default = "default_statsd_port")]
    port: u32
}

//------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct Topic {
    mqtt_topic: String,
    statsd_topic: String
}

// =============================================================================
// Main code
// =============================================================================

fn main() {
    let args = Args::parse();

    let config_str = fs::read_to_string(args.configuration)
        .expect("Failed to read configuration file");
    let config: Config = toml::from_str(&config_str)
        .expect("Error in configuration file");

    println!("{:#?}", config); //TODO: remove

    let mqtt_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(config.mqtt.hostname)
        .finalize();

    let mqtt = mqtt::Client::new(mqtt_opts)
        .expect("Cannot create mqtt client");

    let rx = mqtt.start_consuming();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(20))
        .finalize();

    mqtt.connect(conn_opts)
        .expect( "Cannot connect" );

    for topic in config.topics {
        let e = mqtt.subscribe(topic.mqtt_topic.as_str(), 1);
        if e.is_ok() {
            println!( "Subscribed to {}", topic.mqtt_topic );
        } else if args.verbose {
            println!( "Cannot subscribe to {} (error: {e:?})", topic.mqtt_topic );
        }
    }

    for msg in rx.iter() {
        if let Some(msg) = msg {
            println!("{}", msg);
        } else if !mqtt.is_connected() {
            println!("disconnected");
            break;
        }
    }

    if mqtt.is_connected() {
        println!("Disconnecting");
        //mqtt.unsubscribe_many(topics)
    }
}
