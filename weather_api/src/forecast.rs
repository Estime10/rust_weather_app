use chrono::Utc;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct WeatherResponse {
    #[serde(default)]
    name: String,
    #[serde(rename = "list")]
    forecast: Vec<Forecast>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Forecast {
    #[serde(rename = "dt_txt")]
    date_time: String,
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Main {
    #[serde(rename = "temp")]
    temperature: f32,
    #[serde(rename = "feels_like")]
    feels_like: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Weather {
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct WeatherData<'a> {
    city: &'a str,
    temperature: f64,
    feels_like: f64,
    description: &'a str,
    timestamp: i64,
}

pub (crate) fn forecast() {
    const API_KEY: &str = "69ecca9f44b2498859861bdea6a95b4c";
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

    let mut selected_city = None;
    while selected_city.is_none() {
        println!("Please select a city from the list to get the weather for the next two days :");
        for (i, city) in CITY_NAMES.iter().enumerate() {
            println!("{}. {}", i + 1, city);
        }

        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let selected_index = guess.trim().parse::<usize>();
        match selected_index {
            Ok(index) if index > 0 && index <= CITY_NAMES.len() => {
                selected_city = Some(CITY_NAMES[index - 1])
            }
            _ => println!("Invalid selection, please try again."),
        }
    }

    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}",
        selected_city.unwrap(),
        API_KEY,
    );
    
    let response = reqwest::blocking::get(&url)
        .unwrap()
        .json::<WeatherResponse>()
        .unwrap();

    let forecasts = response.forecast.iter()
        .take(8)
        .step_by(4)
        .map(|forecast| {
            let celsius = forecast.main.temperature - 273.15;
            let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
            format!("{}: {:.1}°C ({:.1}°F), feels like {:.1}°C ({:.1}°F)",
                forecast.date_time,
                celsius,
                fahrenheit,
                forecast.main.feels_like - 273.15,
                forecast.weather[0].description)
        })
        .collect::<Vec<String>>();
    
    println!("Weather forecast for the next two days:");
    println!("{}", forecasts.join("\n"));

    let _weather_data = WeatherData {
        city: &response.name,
        temperature: response.forecast[0].main.temperature as f64 -273.15,
        feels_like: response.forecast[0].main.feels_like as f64 -273.15,
        description: &response.forecast[0].weather[0].description,
        timestamp: Utc::now().timestamp(),
    };

    match insert_weather_data(&_weather_data) {
        Ok(_) => println!("Data inserted"),
        Err(e) => println!("Error inserting data: {}", e),
    }
}

fn insert_weather_data(data: &WeatherData) -> Result<()> {
    let conn = Connection::open("weather.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS forecast (
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
        "INSERT INTO forecast 
        (city, temperature, feels_like, description, timestamp)
        VALUES (?1, ?2, ?3, ?4, ?5)",
        params![data.city, data.temperature, data.feels_like, data.description, data.timestamp],
    )?;
    Ok(())
}
