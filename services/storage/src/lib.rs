//! ## Storage Service
//!
//! Responsible for storing the notification configurations of users.

use rusqlite::{params, Connection, Result};

/*
The storage structure should be as follows:

Each user can have multiple notifications enabled. These notifications must be picked
from the `Notifications` enum. (There cannot be duplicates)

*/

pub fn initialize_db() -> Result<()> {
	let conn = Connection::open("users.db")?;

	conn.execute(
		"CREATE TABLE IF NOT EXISTS users (
			id INTEGER NOT NULL,
			address TEXT NOT NULL,
			username TEXT NOT NULL,
				PRIMARY KEY(id AUTOINCREMENT)
         )",
		(),
	)?;

	// Insert some User
	let (address, username): (String, String) = (String::from("5TR534BDHJSJSNF"), String::from("Jones"));
	conn.execute(
		"INSERT INTO users 
			(address, username) 
			VALUES (?1, ?2)",
		params![username, address]
	)?;

	Ok(())
}
