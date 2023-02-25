use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
struct OneCityData {
    
    city: String,
    temp: String,
    feels_like: String,
    weather_description: String,
    id: i32,
}



impl OneCityData {
    pub(crate) fn insert(
        conn: &Connection,
        city: &str,
        temp: &str,
        feels_like: &str,
        weather_description: &str,
    ) -> Result<()> {
        conn.execute(
            "INSERT INTO one_city (city, temp, feels_like, weather_description)
            VALUES (?, ?, ?, ?)",
            &[&city, &temp, &feels_like, &weather_description],
        )?;
        Ok(())
    }
}

pub (crate) fn one_city() -> Result<()> {


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
    let response = reqwest::blocking::get(&url)
        .unwrap()
        .json::<WeatherResponse>()
        .unwrap();

    // Connect to the database
    let conn = Connection::open("data.db")?;

    let _celsius = response.temperature.temp;
    let city_data = OneCityData {
    city: response.name,
    temp: (response.temperature.temp as f64).to_string(),
    feels_like: (response.temperature.feels_like as f64).to_string(),
    weather_description: response.weather[0].description.clone(),
    id: response.temperature.temp as i32, // assign a unique ID here
};

    // Create the "one_city" table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS one_city (
                  id              INTEGER PRIMARY KEY,
                  city            TEXT NOT NULL,
                  temp            TEXT NOT NULL,
                  feels_like      TEXT NOT NULL,
                  weather_description TEXT NOT NULL
                  )",
                  rusqlite::params![
                    &city_data.id,
                    &city_data.city,
                    &city_data.temp,
                    &city_data.feels_like,
                    &city_data.weather_description,
                ],
    )?;

    // Insert the weather data into the database
    OneCityData::insert(
        &conn,
        &city_data.city,
        &city_data.temp,
        &city_data.feels_like,
        &city_data.weather_description,
    )?;

    Ok(())


}
