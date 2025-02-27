use rusqlite::{params, Connection, Error, Result};
use types::Notifier;

/// The data stored for each user in the database.
#[derive(Debug)]
pub struct User {
	/// A unique identifier for a user.
	pub id: u32,
	/// Email of the user.
	pub email: Option<String>,
	/// Telegram handle of the user.
	pub tg_handle: Option<String>,
	/// Defines the channel through which the user would like to be notified.
	pub notifier: Notifier,
}

impl User {
	pub fn query_all(conn: &Connection) -> Result<Vec<User>> {
		let mut stmt = conn.prepare("SELECT * FROM users WHERE id=?1")?;
		let users_iter = stmt.query_map((), |row| {
			let notifier = match row.get::<_, String>("notifier")?.as_str() {
				"email" => Notifier::Email,
				"telegram" => Notifier::Telegram,
				_ => Notifier::Null,
			};

			Ok(User {
				id: row.get("id")?,
				tg_handle: row.get("tg_handle")?,
				email: row.get("email")?,
				notifier,
			})
		})?;

		let mut users = Vec::new();
		for user in users_iter {
			users.push(user.unwrap());
		}

		Ok(users)
	}

	pub fn query_by_id(conn: &Connection, id: u32) -> Result<User> {
		let mut smth = conn.prepare("SELECT * FROM users WHERE id=?1")?;
		let mut users_iter = smth.query_map(&[&id], |row| {
			let notifier = match row.get::<_, String>("notifier")?.as_str() {
				"email" => Notifier::Email,
				"telegram" => Notifier::Telegram,
				_ => Notifier::Null,
			};
			Ok(User {
				id: row.get("id")?,
				tg_handle: row.get("tg_handle")?,
				email: row.get("email")?,
				notifier,
			})
		})?;

		match users_iter.next() {
			Some(data) => Ok(data.unwrap()),
			None => Err(Error::QueryReturnedNoRows),
		}
	}

	pub fn query_by_email(conn: &Connection, email: String) -> Result<User> {
		let mut smth = conn.prepare("SELECT * FROM users WHERE email=?1")?;
		let mut users_iter = smth.query_map(&[&email], |row| {
			let notifier = match row.get::<_, String>("notifier")?.as_str() {
				"email" => Notifier::Email,
				"telegram" => Notifier::Telegram,
				_ => Notifier::Null,
			};
			Ok(User {
				id: row.get("id")?,
				tg_handle: row.get("tg_handle")?,
				email: row.get("email")?,
				notifier,
			})
		})?;

		match users_iter.next() {
			Some(data) => Ok(data.unwrap()),
			None => Err(Error::QueryReturnedNoRows),
		}
	}

	pub fn query_by_tg_handle(conn: &Connection, handle: String) -> Result<User> {
		let mut smth = conn.prepare("SELECT * FROM users WHERE tg_handle=?1")?;
		let mut users_iter = smth.query_map(&[&handle], |row| {
			let notifier = match row.get::<_, String>("notifier")?.as_str() {
				"email" => Notifier::Email,
				"telegram" => Notifier::Telegram,
				_ => Notifier::Null,
			};
			Ok(User {
				id: row.get("id")?,
				tg_handle: row.get("tg_handle")?,
				email: row.get("email")?,
				notifier,
			})
		})?;

		match users_iter.next() {
			Some(data) => Ok(data.unwrap()),
			None => Err(Error::QueryReturnedNoRows),
		}
	}

	pub fn create_user(conn: &Connection, user: &User) -> Result<()> {
		let User { email, tg_handle, .. } = user;
		let notifier = match user.notifier {
			Notifier::Email => Some("email"),
			Notifier::Telegram => Some("telegram"),
			_ => None,
		};

		match notifier {
			Some(notifier) => {
				conn.execute(
					"INSERT INTO users
                        (email, tg_handle, notifier)
                        VALUES (?1, ?2, ?3)
                    ",
					params![email, tg_handle, notifier],
				)?;
			},
			None => {
				conn.execute(
					"INSERT INTO users
                        (email, tg_handle, notifier)
                        VALUES (?1, ?2, NULL)
                    ",
					params![email, tg_handle],
				)?;
			},
		};
		Ok(())
	}

	pub fn get_connection() -> Result<Connection> {
		let db_path = "db/users.db";

		// Create the `users.db` if it does not exist.
		let conn = Connection::open(db_path)?;
		conn.execute(
			"CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                tg_handle TEXT UNIQUE,
                email TEXT UNIQUE,
                notifier TEXT CHECK (
                    notifier IN ('email', 'telegram') 
                    OR notifier IS NULL
                )
            )",
			(),
		)?;

		Ok(conn)
	}
}
