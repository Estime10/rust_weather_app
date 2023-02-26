use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection,Result};



#[derive(Debug, Deserialize, Serialize)]
struct WeatherResponse {
    name: String,
    #[serde(rename = "main")]
    temperature: Main,
    weather: Vec<Weather>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Main {
    temp: f64,
    #[serde(rename = "feels_like")]
    feels_like: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Weather {
description: String,
}
// struc for the weather data in the database
#[derive(Debug, Deserialize, Serialize)]
struct WeatherData<'a>{
    city: &'a str,
    temperature: f64,
    feels_like: f64,
    description: &'a str,

}

pub (crate) fn one_city() {
    // Get the weather data for a city
    const API_KEY: &str = "69ecca9f44b2498859861bdea6a95b4c";
    println!("Enter a city name:");

    let mut guess = String::new();

    std::io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let city_name = guess.trim();

    let url = format!(

        "http://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}",
        city_name, API_KEY
    );
    let _response: WeatherResponse = reqwest::blocking::get(&url)
        .unwrap()
        .json::<WeatherResponse>()
        .unwrap();
    println!("{:#?}", _response);

    let _weather_data = WeatherData {
        city: &_response.name,
        temperature:_response.temperature.temp,
        feels_like:_response.temperature.feels_like,
        description: &_response.weather[0].description,
    };
    match insert_weather_data(&_weather_data) {
        Ok(_) => println!("Data inserted"),
        Err(e) => println!("Error inserting data: {}", e),
    }
    
}
fn insert_weather_data(data:&WeatherData) -> Result<()> {
    let conn = Connection::open("weather.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS weather_data (
            id INTEGER PRIMARY KEY,
            city TEXT NOT NULL,
            temperature REAL NOT NULL,
            feels_like REAL NOT NULL,
            description TEXT NOT NULL
        )",
        params![],
    )?;
    
    conn.execute(
        "INSERT INTO weather_data (city, temperature, feels_like, description)
        VALUES (?1, ?2, ?3, ?4)",
        params![data.city, data.temperature, data.feels_like, data.description],
    )?;
    Ok(())
}

