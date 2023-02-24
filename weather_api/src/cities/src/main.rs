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
    // api key and country name are constants
    const API_KEY: &str = "69ecca9f44b2498859861bdea6a95b4c";
    const COUNTRY_NAME: &str = "Belgium";

    // CITY_NAMES is an array of strings
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

    // Print the current weather for each city
    for city_name in &CITY_NAMES {
        // url is the api url
        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
            city_name, COUNTRY_NAME, API_KEY
        );

        // response is a WeatherResponse struct
        let response = reqwest::blocking::get(&url)
            .unwrap()
            // json deserializes the response body into a WeatherResponse struct
            .json::<WeatherResponse>()
            // unwrap returns the value of the result or panics if the result is an error
            .unwrap();

        // Convert the temperature from Kelvin to Celsius
        let celsius = response.temperature.temp;

        let message = if celsius <= 0.0 {
            "Go get yourself a pair of gloves at Decathlon"
        } else if celsius <= 6.0 {
            "Go get yourself a rain coat at Decathlon"
        } else {
            "The weather is fine but don't forget to hydrate yourself with a bottle of water at Decathlon"
        };

        // Convert the temperature from Celsius to Fahrenheit
        let fahrenheit = celsius * 9.0 / 5.0 + 32.0;

        // print the current weather for each city in celsius and fahrenheit
        println!(
            "Current weather in {}: {:.1}°C ({:.1}°F), feels like {:.1}°C ({:.1}°F), {}",
            response.name,
            celsius,
            fahrenheit,
            response.temperature.feels_like,
            response.weather[0].description,
            message
        );
    }
}
