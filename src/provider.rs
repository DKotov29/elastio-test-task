use reqwest::blocking::Response;
use serde_json::Value;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;

pub trait Provider {
    fn get(&self, address: String) -> WeatherData;
}

pub struct WeatherData {
    temp_c: f64,
    wind_kph: f64,
    humidity_percent: u8,
    clouds_percent: u8,
}

impl Display for WeatherData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "temperature: {}", self.temp_c)?;
        writeln!(f, "wind in kph: {}", self.wind_kph)?;
        writeln!(f, "hunidity: {}%", self.humidity_percent)?;
        writeln!(f, "cloud cover: {}%", self.clouds_percent)
    }
}

pub struct WeatherApiProvider {
    pub api_key: String,
}
pub struct OpenWeatherProvider {
    pub api_key: String,
}

impl Provider for OpenWeatherProvider {
    fn get(&self, address: String) -> WeatherData {
        let mut response = reqwest::blocking::get(format!(
            "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
            address, self.api_key
        ))
        .unwrap();
        let response_code = response.status();

        let mut value = &response.json::<Value>().unwrap();
        if response_code.as_u16() != 200 {
            panic!("{}", value["message"])
        }
        response = reqwest::blocking::get(format!(
            "https://api.openweathermap.org/data/2.5/onecall?lat={}&lon={}&units=metric&exclude=hourly,daily&appid={}",
            value[0]["lat"],value[0]["lon"], self.api_key
        )).unwrap();
        let mut value = &response.json::<Value>().unwrap();
        if response_code.as_u16() != 200 {
            panic!("{}", value["message"])
        }
        WeatherData {
            temp_c: value["current"]["temp"].as_f64().unwrap() as f64,
            wind_kph: value["current"]["wind_speed"].as_f64().unwrap(),
            humidity_percent: value["current"]["humidity"].as_u64().unwrap() as u8,
            clouds_percent: value["current"]["clouds"].as_u64().unwrap() as u8,
        }
    }
}
impl Provider for WeatherApiProvider {
    fn get(&self, address: String) -> WeatherData {
        let mut response = reqwest::blocking::get(format!(
            "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
            self.api_key, address
        ))
        .unwrap();
        let response_code = response.status();

        let value = &response.json::<Value>().unwrap();
        if response_code.as_u16() != 200 {
            panic!("{}", value["message"])
        }
        WeatherData {
            temp_c: value["current"]["temp_c"].as_f64().unwrap(),
            wind_kph: value["current"]["wind_kph"].as_f64().unwrap(),
            humidity_percent: value["current"]["humidity"].as_u64().unwrap() as u8,
            clouds_percent: value["current"]["cloud"].as_u64().unwrap() as u8,
        }
    }
}
