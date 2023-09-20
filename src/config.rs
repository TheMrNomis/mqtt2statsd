use std::error::Error;
use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

// =============================================================================
// Structs
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub statsd: StatsdConfig,
    pub topics: Vec<Topic>
}

//------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct MqttConfig {
    pub hostname: String,

    #[serde(default = "default_mqtt_port")]
    pub port: u32,

    #[serde(default = "default_mqtt_timeout")]
    pub timeout: u32
}

//------------------------------------------------------------------------------


#[derive(Debug, Deserialize)]
pub struct StatsdConfig {
    pub hostname: String,

    #[serde(default = "default_statsd_prefix")]
    pub prefix: String,

    #[serde(default = "default_statsd_port")]
    pub port: u32
}

//------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct Topic {
    pub mqtt: String,
    pub statsd: String
}

// =============================================================================
// Public Functions
// =============================================================================

pub fn from_file( path: &PathBuf ) -> Result<Config, Box<dyn Error>> {
    let config_str = fs::read_to_string(path)?;
    Ok( toml::from_str(&config_str)? )
}

// =============================================================================
// Private Functions
// =============================================================================

fn default_mqtt_port() -> u32 {
    1883
}

//------------------------------------------------------------------------------

fn default_mqtt_timeout() -> u32 {
    60
}

//------------------------------------------------------------------------------

fn default_statsd_port() -> u32 {
    8125
}

//------------------------------------------------------------------------------

fn default_statsd_prefix() -> String {
    String::from("mqtt.stats")
}
