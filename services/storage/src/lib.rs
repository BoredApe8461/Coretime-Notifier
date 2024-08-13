//! ## Storage Service
//!
//! Responsible for storing the notification configurations of users.

use rusqlite::{params, Connection, Result};

/*
The storage structure should be as follows:

Each user can have multiple notifications enabled. These notifications must be picked
from the `Notifications` enum. (There cannot be duplicates)

*/
pub mod users;

pub fn initialize_db() -> Result<()> {
	let conn = Connection::open("users.db")?;

	conn.execute(
		"CREATE TABLE IF NOT EXISTS users (
			id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
			tg_handle TEXT,
			email TEXT,
			notifier TEXT CHECK (
				notifier IN ('email', 'telegram') 
				OR notifier IS NULL
			)
         )",
		(),
	)?;

	// Insert some User
	let (address, username): (String, String) = (String::from("5TR534BDHJSJSNF"), String::from("Jones"));
	conn.execute(
		"INSERT INTO users 
			(email, tg_handle) 
			VALUES (?1, ?2)",
		params![username, address]
	)?;


	Ok(())
}
