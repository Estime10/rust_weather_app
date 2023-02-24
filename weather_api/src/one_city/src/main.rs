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

pub fn main() {
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
