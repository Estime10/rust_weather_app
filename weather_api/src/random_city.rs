

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
    temp: f32,
    #[serde(rename = "feels_like")]
    feels_like: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Weather {
    description: String,
}

pub (crate) fn random_city() {
    // api key and country name are constants
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
    let mut selected_city = None;
    while selected_city.is_none() {
        println!("Please select a city from the following list:");
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
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        selected_city.unwrap(), COUNTRY_NAME, API_KEY
    );
    let response = reqwest::blocking::get(&url)
        .unwrap()
        .json::<WeatherResponse>()
        .unwrap();
    let celsius = response.temperature.temp;
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!(
        "Current weather in {}: {:.1}°C ({:.1}°F), feels like {:.1}°C ({:.1}°F), {}",
        response.name,
        celsius,
        fahrenheit,
        response.temperature.feels_like,
        response.weather[0].description,
        response.weather[0].description
    );
}
