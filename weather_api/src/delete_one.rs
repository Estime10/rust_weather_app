use rusqlite::{Connection, Result};

struct ForecastData {
    id: i32,
    city: String,
    timestamp: i64,
    temperature: f64,
    feels_like: f64,
    description: String,
}

pub(crate) fn delete_forecast(id: i32) -> Result<()> {
    let conn = Connection::open("weather.db")?;
    let mut stmt = conn.prepare("DELETE FROM forecast WHERE id = ?")?;
    stmt.execute(&[&id])?;
    Ok(())
}

pub(crate) fn delete_one() -> Result<()> {
    let conn = Connection::open("weather.db")?;
    let mut stmt = conn.prepare("SELECT * FROM forecast")?;
    let rows = stmt.query_map([], |row| {
        Ok(ForecastData {
            id: row.get(0)?,
            city: row.get(1)?,
            temperature: row.get(2)?,
            feels_like: row.get(3)?,
            description: row.get(4)?,
            timestamp: row.get(5)?,
        })
    })?;

    let forecast_data = rows.into_iter()
    .collect::<Result<Vec<_>, _>>()?;

    for data in &forecast_data {
        println!("ID: {}", data.id);
        println!("City: {}", data.city);
        println!("Temperature: {}", data.temperature);
        println!("feels_like: {}", data.feels_like);
        println!("description: {}", data.description);
        println!("timestamp: {}", data.timestamp);
    }

    println!("Please choose a forecast data by ID:");
    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    match guess.trim().parse::<i32>() {
        Ok(selected_id) => match forecast_data.iter().find(|&data| data.id == selected_id) {
            Some(data) => {
                println!("ID: {}", data.id);
                println!("City: {}", data.city);
                println!("Temperature: {}", data.temperature);
                println!("feels_like: {}", data.feels_like);
                println!("description: {}", data.description);
                println!("timestamp: {}", data.timestamp);

                println!("Are you sure you want to delete this forecast? (y/n)");
                let mut response = String::new();
                std::io::stdin()
                    .read_line(&mut response)
                    .expect("Failed to read line");

                if response.trim().to_lowercase() == "y" {
                    delete_forecast(selected_id)?;
                    println!("Forecast deleted.");
                } else {
                    println!("Forecast not deleted.");
                }
            }
            None => println!("Invalid ID, please try again."),
        },
        Err(_) => println!("Invalid input, please enter a number."),
    }

    Ok(())
}
