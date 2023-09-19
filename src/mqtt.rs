extern crate paho_mqtt as paho;

use crate::config::MqttConfig;
use crate::config::Topic;

// =============================================================================
// Public Functions
// =============================================================================

pub fn create( config: &MqttConfig ) -> Result<paho::Client, paho::Error> {
    let client_opts = paho::CreateOptionsBuilder::new()
        .server_uri(&config.hostname)
        .finalize();

    let ret = paho::Client::new(client_opts)?;

    let conn_opts = paho::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(20))
        .finalize();

    ret.connect(conn_opts)?;

    Ok(ret)
}

//------------------------------------------------------------------------------

pub fn subscribe( mqtt: &paho::Client, topics: &Vec<Topic>, verbose: bool ) {

    for topic in topics {
        let e = mqtt.subscribe(topic.mqtt_topic.as_str(), 1);
        if e.is_ok() {
            println!( "Subscribed to {}", topic.mqtt_topic );
        } else if verbose {
            println!( "Cannot subscribe to {} (error: {e:?})", topic.mqtt_topic );
        }
    }
}
