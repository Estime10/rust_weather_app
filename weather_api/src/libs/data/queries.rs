use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct OneCityData {
    
    pub city: String,
    pub temp: String,
    pub feels_like: String,
    pub weather_description: String,
}

impl OneCityData {
    pub fn insert(
        conn: &Connection,
        city: &str,
        temp: &str,
        feels_like: &str,
        weather_description: &str,
    ) -> Result<()> {
        conn.execute(
            "INSERT INTO one_city (city, temp, feels_like, weather_description)
                  VALUES (?1, ?2, ?3, ?4)",
            &[&city, &temp, &feels_like, &weather_description],
        )?;
        Ok(())
    }
}
