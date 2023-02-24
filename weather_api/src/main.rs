use serde::Deserialize;
pub mod selected;
pub mod forecast;
pub mod cities;


#[derive(Deserialize)]
struct WeatherResponse {
    name: String,
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
    feels_like: f32,
}

#[derive(Deserialize)]
struct Weather {
    description: String,
}

fn main() {
    

}
