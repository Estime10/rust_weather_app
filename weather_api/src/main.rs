
// This is the main file for the weather_api project. It is the entry point for the program. 

pub(crate) mod ten_cities;
pub(crate) mod random;
pub(crate) mod forecast;
pub(crate) mod one_city;




fn main() {
    println!("main");
    ten_cities::ten_cities(); //ten cities in Belgium database table -added succefully-
    one_city::one_city(); //one city in belgium database table -added succefully- 
    random::random();  //random city in the world database table -added succefully-
    forecast::forecast(); //forecast for the next two days for a city selected by the user no -table yet-
  }
pub fn ten_cities() {
    println!("ten_cities");
}
pub fn one_city() {
    println!("one_city");
}
pub fn random_city() {
    println!("random_city");
}
pub fn forecast() {
    println!("forecast");
}
