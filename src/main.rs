use std::error::Error;

mod cli_args;
mod config;
mod mqtt;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli_args::parse();
    let config = config::from_file(&args.configuration)?;

    println!("{:#?}", config); //TODO: remove

    let mqtt = mqtt::create( &config.mqtt )?;
    let rx = mqtt.start_consuming();
    mqtt::subscribe( &mqtt, &config.topics, args.verbose );

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

    Ok(())
}
