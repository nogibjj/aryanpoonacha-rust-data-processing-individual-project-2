extern crate csv;
extern crate rusqlite;

use csv::Reader;
use rusqlite::{params, Connection};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let dataset = "tables/cars.csv";

    // Get the current directory
    let current_dir = env::current_dir()?;
    println!("{:?}", current_dir);

    // Open the CSV file
    let mut rdr = Reader::from_path(dataset)?;

    // Connect to the SQLite database
    let conn = Connection::open("CarsDB.db")?;

    // Drop the CarsDB table if it exists
    conn.execute("DROP TABLE IF EXISTS CarsDB", params![])?;

    // Create the CarsDB table
    conn.execute(
        "CREATE TABLE CarsDB (
            car_name TEXT PRIMARY KEY,
            mpg REAL,
            cyl INTEGER,
            disp REAL,
            hp INTEGER,
            drat REAL,
            wt REAL,
            qsec REAL,
            vs INTEGER,
            am INTEGER,
            gear INTEGER,
            carb INTEGER
        )",
        params![],
    )?;

    // Insert data into the CarsDB table
    for result in rdr.records() {
        let record = result?;
        conn.execute(
            "INSERT INTO CarsDB (car_name, mpg, cyl, disp, hp, drat, wt, qsec, vs, am, gear, carb)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                &record[0],
                record[1].parse::<f64>().unwrap(),
                record[2].parse::<i32>().unwrap(),
                record[3].parse::<f64>().unwrap(),
                record[4].parse::<i32>().unwrap(),
                record[5].parse::<f64>().unwrap(),
                record[6].parse::<f64>().unwrap(),
                record[7].parse::<f64>().unwrap(),
                record[8].parse::<i32>().unwrap(),
                record[9].parse::<i32>().unwrap(),
                record[10].parse::<i32>().unwrap(),
                record[11].parse::<i32>().unwrap()
            ],
        )?;
    }

    Ok(())
}
