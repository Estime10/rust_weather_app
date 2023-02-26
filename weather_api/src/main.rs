pub(crate) mod ten_cities;
pub(crate) mod random;
pub(crate) mod forecast;
pub(crate) mod one_city;
pub(crate) mod pick_one;
pub(crate) mod delete_one;

fn main()  {
    println!("main");
    ten_cities::ten_cities(); //ten cities in Belgium database table -added succefully-
    one_city::one_city(); //one city in belgium database table -added succefully-
    random::random();  //random city in the world selected by the user database table -added succefully-
    forecast::forecast(); //forecast for the next two days for a city selected in the list of cities in the database table -added succefully-
    match pick_one::pick_one() {
        Ok(_) => println!("pick_one completed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    } //pick one city from the database table 
    match delete_one::delete_one() {
        Ok(_) => println!("delete_one completed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    } //delete one city from the database table
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
pub fn pick_one() {
    println!("pick_one");
}
