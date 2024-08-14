//! ## Storage Service
//!
//! Responsible for storing the notification configurations of users.
/*
The storage structure should be as follows:

Each user can have multiple notifications enabled. These notifications must be picked
from the `Notifications` enum. (There cannot be duplicates)

*/
pub mod users;
