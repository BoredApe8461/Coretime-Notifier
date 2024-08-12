use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct User {
	pub id: u32,
	pub address: String,
	pub username: String,
}

impl User {
    fn query_all(conn: &Connection) -> Result<Vec<User>> {
        let mut smth = conn.prepare("SELECT * FROM users WHERE id=?1")?;
        let users_iter = smth.query_map((), |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                address: row.get(2)?,
            })
        })?;
        
        let mut users = Vec::new();
        for user in users_iter {
            users.push(user.unwrap());
        }

        Ok(users)
    }

    fn query_by_id(conn: &Connection, id: u32) -> Result<User> {
        let mut smth = conn.prepare("SELECT * FROM users WHERE id=?1")?;
        let mut users_iter = smth.query_map(&[&id], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                address: row.get(2)?,
            })
        })?;

        let user = users_iter.next().unwrap().unwrap();
        Ok(user)
    }
}