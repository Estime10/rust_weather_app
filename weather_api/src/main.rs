
// This is the main file for the weather_api project. It is the entry point for the program. 

pub(crate) mod ten_cities;
pub(crate) mod one_city;
pub(crate) mod forecast;
pub(crate) mod random_city;




fn main() {
    println!("main");
    ten_cities::get_city();
    one_city::one_city();
    random_city::random_city();
    forecast::forecast();
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
