use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct WeatherResponse {
    #[serde(default)]
    name: Option<String>,
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

    if let Some(name) = response.name {
        println!("Weather forecast for {}:", name);
    } else {
        println!("Weather forecast:");
    }

    for forecast in response.forecast.iter().take(8).step_by(4) {
        let celsius = forecast.main.temperature - 273.15;
        let fahrenheit = celsius * 9.0 / 5.0 + 32.0;

        println!(
            "{}: {:.1}°C ({:.1}°F), feels like {:.1}°C ({:.1}°F)",
            forecast.date_time,
            celsius,
            fahrenheit,
            forecast.main.feels_like - 273.15,
            forecast.weather[0].description,
        );
    }
}


