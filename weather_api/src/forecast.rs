use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]

// struct weatherResponse is the root of the JSON response from the API and contains the name of the city and the forecast
struct WeatherResponse {
    // name is the name of the city in the response JSON
    #[serde(default)]
    name: Option<String>,
    // forecast is the list of forecasts in the response JSON
    #[serde(rename = "list")]
    forecast: Vec<Forecast>,
}

#[derive(Debug, Deserialize, Serialize)]
// struct Forecast contains the date and time of the forecast, the main weather information and the weather description
struct Forecast {
    // date_time is the date and time of the forecast in the response JSON (e.g. 2021-03-01 12:00:00) 
    #[serde(rename = "dt_txt")]
    date_time: String,
    // main is the main weather information in the response JSON
    main: Main,
    // weather is the weather description in the response JSON
    weather: Vec<Weather>,
}

#[derive(Debug, Deserialize, Serialize)]
// struct Main contains the temperature and the "feels like" temperature in the response JSON 
struct Main {
    
    #[serde(rename = "temp")]
    // temperature is the temperature in the response JSON
    temperature: f32,
    #[serde(rename = "feels_like")]
    // feels_like is the "feels like" temperature in the response JSON
    feels_like: f32,
}

#[derive(Debug, Deserialize, Serialize)]
// struct Weather contains the weather description in the response JSON
struct Weather {
    // description is the weather description in the response JSON
    description: String,
}
// fn forecast() is the function that gets the weather forecast for the next two days for a city selected by the user
pub (crate) fn forecast() {
    // API_KEY is the API key for the OpenWeatherMap API
    const API_KEY: &str = "69ecca9f44b2498859861bdea6a95b4c";
    // CITY_NAMES is the list of cities for which the weather forecast can be retrieved
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
    // let mut selected_city is the city selected by the user
    let mut selected_city = None;
    // while selected_city.is_none() is a loop that runs until the user selects a city from the list
    while selected_city.is_none() {
        // println!() prints the list of cities for which the weather forecast can be retrieved
        println!("Please select a city from the list to get the weather for the next two days :");
    // for (i, city) in CITY_NAMES.iter().enumerate() is a loop that prints the list of cities for which the weather forecast can be retrieved
        for (i, city) in CITY_NAMES.iter().enumerate() {
            // println!() prints the list of cities for which the weather forecast can be retrieved 
            println!("{}. {}", i + 1, city);
        }
// let mut guess is the user input 
        let mut guess = String::new();
// std::io::stdin().read_line(&mut guess).expect("Failed to read line"); is a function that reads the user input
        std::io::stdin()
// .read_line(&mut guess) is a function that reads the user input
            .read_line(&mut guess)
// .expect("Failed to read line") is a function that reads the user input
            .expect("Failed to read line");
// let selected_index = guess.trim().parse::<usize>(); is a function that reads the user input
        let selected_index = guess.trim().parse::<usize>();
// match selected_index  is a function that reads the user input
        match selected_index {
// function that reads the user input and checks if the user input is valid 
            Ok(index) if index > 0 && index <= CITY_NAMES.len() => {
                selected_city = Some(CITY_NAMES[index - 1])
            }
// _ => println!("Invalid selection, please try again.") is a function that reads the user input and checks if the user input is valid 
            _ => println!("Invalid selection, please try again."),
        }
    }
// url is the url for the OpenWeatherMap API
    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}",
        selected_city.unwrap(),
        API_KEY,
        
    );
// response is the response from the OpenWeatherMap API request 
    let response = reqwest::blocking::get(&url)
        .unwrap()
        .json::<WeatherResponse>()
        .unwrap();
// if let Some(name) = response.name is a function that prints the weather forecast for the next two days for the city selected by the user
    if let Some(name) = response.name {
        println!("Weather forecast for {}:", name);
    } else {
        println!("Weather forecast:");
    }
// for line is a function that prints the weather forecast for the next two days for the city selected by the user
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


