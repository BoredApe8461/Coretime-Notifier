//! ## Storage Service
//!
//! Responsible for storing the notification configurations of users.

use rusqlite::{Connection, Result};

/*
The storage structure should be as follows:

Each user can have multiple notifications enabled. These notifications must be picked
from the `Notifications` enum. (There cannot be duplicates)

*/

pub fn initialize_db() -> Result<()> {
    let conn = Connection::open("cats.db")?;

    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
         ()
    )?;

    Ok(())
}
