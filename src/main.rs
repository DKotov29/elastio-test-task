use crate::provider::Provider;
use clap::{arg, Command};
use config::Config;
use std::fs;

mod provider;

fn main() {
    let matches = Command::new("weather")
        .about("test weather app")
        .arg_required_else_help(true)
        .author("https://github.com/DKotov29")
        .subcommand(
            Command::new("configure").about("configure provider").arg(
                arg!(<PROVIDER>)
                    .help("weather data provider")
                    .possible_values(["WeatherApi", "OpenWeather"]),
            ),
        )
        .subcommand(
            Command::new("get")
                .about("get weather data from previously selected provider")
                .arg(arg!(<ADDRESS>).help("address of interest").required(true))
                .arg(arg!(--date <DATE>).required(false)),
        )
        .get_matches();
    let config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();
    match matches.subcommand() {
        Some(("configure", configure_matches)) => {
            fs::write(
                "config.toml",
                format!(
                    "provider = \"{}\"\napi_key = \"{}\"",
                    configure_matches.value_of("PROVIDER").unwrap(),
                    config.get_string("api_key").unwrap()
                ),
            )
            .expect("Could not write to file!");
        }
        Some(("get", get_matches)) => {
            let provider_name: String = config
                .get_string("provider")
                .expect("unexpected provider in config");
            let api_key: String = config.get_string("api_key").expect("api key not found");
            match Provider::try_from((provider_name, api_key)) {
                Ok(provider_res) => {
                    match provider_res.get(get_matches.value_of("ADDRESS").unwrap().to_string()) {
                        Ok(weather_data) => {
                            println!("{}", weather_data);
                        }
                        Err(err) => {
                            println!("{:?}", err);
                        }
                    }
                }
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        }
        _ => println!("unpredictable move"),
    };
}
