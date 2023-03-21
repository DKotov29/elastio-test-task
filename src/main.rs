use crate::provider::Provider;
use clap::{arg, Command};
use config::Config;
use std::{fs, io};

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
        .expect("config building failed");
    match matches.subcommand() {
        Some(("configure", configure_matches)) => {
            let mut api_key = String::new();
            io::stdin().read_line(&mut api_key).expect("i/o error");
            api_key = api_key.trim().to_string();
            let provider = match configure_matches.value_of("PROVIDER") {
                None => {
                    println!("provider not provided. example of right usage of program: program.exe configure openweather");
                    return;
                }
                Some(provider_name) => {
                    let provider_name = provider_name.to_lowercase();
                    if provider_name.as_str() != "weatherapi"
                        || provider_name.as_str() != "openweather"
                    {
                        println!("this provider is not supported now. we support only weatherapi and weatherapi now")
                    }
                    provider_name
                }
            };
            fs::write(
                "config.toml",
                format!("provider = \"{}\"\napi_key = \"{}\"", provider, api_key),
            )
            .expect("Could not write to file!");
        }
        Some(("get", get_matches)) => {
            let provider_name: String = config
                .get_string("provider")
                .expect("provider not found in config");
            let api_key: String = config
                .get_string("api_key")
                .expect("api key not found in config");
            match Provider::try_from((provider_name, api_key)) {
                Ok(provider_res) => {
                    let address = match get_matches.value_of("ADDRESS") {
                        None => {
                            println!("address is not provided. example of right usage of program: program.exe get London");
                            return;
                        }
                        Some(x) => x.to_string(),
                    };
                    match provider_res.get(address) {
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
