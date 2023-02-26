use chrono::Utc;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Main {
    temp: f32,
    #[serde(rename = "feels_like")]
    feels_like: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Weather {
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct WeatherResponse {
    name: String,
    #[serde(rename = "main")]
    temperature: Main,
    weather: Vec<Weather>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WeatherData<'a> {
    city: &'a str,
    temperature: f64,
    feels_like: f64,
    description: &'a str,
    timestamp: i64,
}

pub (crate) fn ten_cities() {
    
    const API_KEY: &str = "69ecca9f44b2498859861bdea6a95b4c";
const COUNTRY_NAME: &str = "Belgium";
const CITY_NAMES: [&str; 10] = [
    "Brussels",
    "Liege",
    "Verviers",
    "Anvers",
    "Ostende",
    "Wevelgem",
    "Sint-Niklaas",
    "Leuven",
    "Charleroi",
    "Aalst",
];
    for city_name in &CITY_NAMES {
        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
            city_name, COUNTRY_NAME, API_KEY
        );
        let response = reqwest::blocking::get(&url)
        .unwrap()
        .json::<WeatherResponse>()
        .unwrap();
        let celsius = response.temperature.temp;
        let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
        println!(
            "Current weather in {}: {:.1}°C ({:.1}°F), feels like {:.1}°C ({:.1}°F)",
            response.name,
            celsius,
            fahrenheit,
            response.temperature.feels_like,
            response.temperature.feels_like * 9.0 / 5.0 + 32.0
        );
        let _response: WeatherResponse = reqwest::blocking::get(&url)
            .unwrap()
            .json::<WeatherResponse>()
            .unwrap();
        println!("{:#?}", _response);

        let _weather_data = WeatherData {
            city: &response.name,
            temperature: celsius as f64,
            feels_like: response.temperature.feels_like as f64,
            description: &response.weather[0].description,
            timestamp: Utc::now().timestamp(),
        };

        match insert_weather_data(&_weather_data) {
            Ok(_) => println!("Data inserted"),
            Err(e) => println!("Error inserting data: {}", e),
        }
    }
}

fn insert_weather_data(data: &WeatherData) -> Result<()> {
    let conn = Connection::open("weather.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ten_cities (
            id INTEGER PRIMARY KEY,
            city TEXT NOT NULL,
            temperature REAL NOT NULL,
            feels_like REAL NOT NULL,
            description TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        )",
        params![],
    )?;
    conn.execute(
        "INSERT INTO ten_cities 
        (city, temperature, feels_like, description, timestamp)
        VALUES (?1, ?2, ?3, ?4, ?5)",
        params![data.city, data.temperature, data.feels_like, data.description, data.timestamp],
    )?;
    Ok(())
}

