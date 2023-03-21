use crate::provider::ProviderCreationError::NotImplemented;
use crate::provider::ProviderUsingError::Get;
use crate::Provider::{OpenWeather, WeatherApi};
use reqwest::Error;
use serde_json::Value;
use std::fmt::{Display, Formatter};

pub enum Provider {
    WeatherApi { api_key: String },
    OpenWeather { api_key: String },
}

#[derive(Debug)]
pub enum ProviderCreationError {
    NotImplemented,
}

#[derive(Debug)]
pub enum ProviderUsingError {
    Get(Error),
    BadResponse(String),
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
        writeln!(f, "humidity: {}%", self.humidity_percent)?;
        writeln!(f, "cloud cover: {}%", self.clouds_percent)
    }
}

impl From<Error> for ProviderUsingError {
    fn from(e: Error) -> Self {
        Get(e)
    }
}

impl TryFrom<(String, String)> for Provider {
    type Error = ProviderCreationError;
    fn try_from((provider, api_key): (String, String)) -> Result<Provider, ProviderCreationError> {
        match provider.to_lowercase().as_str() {
            "openweather" => Ok(OpenWeather { api_key }),
            "weatherapi" => Ok(WeatherApi { api_key }),
            _ => Err(NotImplemented),
        }
    }
}
const BAD_MSG: &str = "if you see this, we have problem with retrieved data. maybe api was changed, so you can contact dev";

impl Provider {
    pub fn get(&self, address: String) -> Result<WeatherData, ProviderUsingError> {
        match self {
            WeatherApi { api_key } => {
                let response = reqwest::blocking::get(format!(
                    "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
                    api_key, address
                ))?;
                let response_status = response.status();
                let value = response.json::<Value>()?;
                if !response_status.is_success() {
                    return Err(ProviderUsingError::BadResponse(
                        value["message"].to_string(),
                    ));
                }
                Ok(WeatherData {
                    temp_c: value["current"]["temp_c"].as_f64().expect(""),
                    wind_kph: value["current"]["wind_kph"].as_f64().expect(BAD_MSG),
                    humidity_percent: value["current"]["humidity"].as_u64().expect(BAD_MSG) as u8,
                    clouds_percent: value["current"]["cloud"].as_u64().expect(BAD_MSG) as u8,
                })
            }
            OpenWeather { api_key } => {
                let mut response = reqwest::blocking::get(format!(
                    "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
                    address, api_key
                ))?;
                let mut response_status = response.status();
                let mut value = response.json::<Value>()?;
                if !response_status.is_success() {
                    return Err(ProviderUsingError::BadResponse(
                        value["message"].to_string(),
                    ));
                }
                response = reqwest::blocking::get(format!(
                    "https://api.openweathermap.org/data/2.5/onecall?lat={}&lon={}&units=metric&exclude=hourly,daily&appid={}",
                    value[0]["lat"], value[0]["lon"], api_key
                ))?;
                response_status = response.status();
                value = response.json::<Value>()?;
                if !response_status.is_success() {
                    return Err(ProviderUsingError::BadResponse(
                        value["message"].to_string(),
                    ));
                }
                Ok(WeatherData {
                    temp_c: value["current"]["temp"].as_f64().expect(BAD_MSG),
                    wind_kph: value["current"]["wind_speed"].as_f64().expect(BAD_MSG),
                    humidity_percent: value["current"]["humidity"].as_u64().expect(BAD_MSG) as u8,
                    clouds_percent: value["current"]["clouds"].as_u64().expect(BAD_MSG) as u8,
                })
            }
        }
    }
}
