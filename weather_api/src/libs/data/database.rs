pub mod database;

fn main() {
    let conn = database::connect();
    let city = database::get_city(&conn, 1);
    println!("City: {}", city.name);
}