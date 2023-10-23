extern crate rusqlite;

use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    // Connect to the SQLite database
    let conn = Connection::open("CarsDB.db")?;

    // Query to find cars with the most horsepower
    let mut stmt = conn.prepare("
        SELECT car_name, hp
        FROM CarsDB
        ORDER BY hp DESC
        LIMIT 5
    ")?;

    let rows = stmt.query_map(params![], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?
        ))
    })?;

    // Print the query results
    println!("Cars with the Most Horsepower:");
    println!("{:<20} {:<10}", "Car Name", "HP");
    println!("{}", "-".repeat(30));

    for row in rows {
        let (car_name, hp): (String, i32) = row?;
        println!("{:<20} {:<10}", car_name, hp);
    }

    Ok(())
}