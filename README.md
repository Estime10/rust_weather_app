
# Weather App in Rust

This Rust application is a simple weather app that allows you to retrieve weather information
for 10 cities in Belgium, as well as a random city from around the world. 
It uses SQLite3 to store forecast information for the selected cities

## Getting Started

1. Clone the repository onto your local machine.
2. Open a terminal window and navigate to the project directory.
3. Ensure you have Rust and SQLite3 installed on your machine.
4. Run the command sqlite3 weather.db in your terminal to create the SQLite3 database.
5. Run the command cargo run to start the application.

### Features

The application currently supports the following features:

- Retrieving weather information for 10 cities in Belgium, stored in the ten_cities database.
- Retrieving weather information for one randomly selected city from around the world, stored in the random database.
- Storing forecast information for selected cities in the weather.db database.
- Retrieving forecast information for one city from the forecast table.
- Selecting and deleting one city from the forecast table.










## Usage/Examples
When you run the application, you will be prompted to select an option from the following menu:
```javascript
Please select an option:

1. Get weather for 10 cities in Belgium
2. Get weather for one random city in the world
3. Store forecast information for a city
4. Retrieve forecast information for a city
5. Select and delete a city from the forecast table y or n question
6. Quit
```


## Running Tests

To run tests, run the following command

```bash
  Cargo run 
  will print all the features one after the other since I didn't find a way to
  separate the files 
```



## Documentation

[Documentation](https://linktodocumentation)

Option 1: Get weather for 10 cities in Belgium
This option will retrieve weather information for 10 cities in Belgium and display it in the terminal.

Option 2: Get weather for one random city in the world
This option will retrieve weather information for one randomly selected city from around the world and display it in the terminal.

Option 3: Store forecast information for a city
This option will prompt you to enter a city name and its corresponding forecast information. The information will be stored in the weather.db database.

Option 4: Retrieve forecast information for a city
This option will prompt you to enter a city ID. The forecast information for the corresponding city will be retrieved from the forecast table and displayed in the terminal.

Option 5: Select and delete a city from the forecast table
This option will prompt you to enter a city ID. The corresponding city will be deleted from the forecast table.

Option 6: Quit
This option will exit the application.

Conclusion
This Rust application is a simple weather app that allows you to retrieve weather information for 10 cities in Belgium, as well as a random city from around the world. It also allows you to store and retrieve forecast information for selected cities using SQLite3. Feel free to explore and modify the code to suit your needs.




## Tech Stack



**Server:** Rust
sqlite3
Cargo





