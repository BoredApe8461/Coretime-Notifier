use rocket::{post, serde::json::Json};
use serde::{Deserialize, Serialize};
use types::{Notifications, Notifier};

use storage::users::User;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegistrationData {
	// TODO, for now we are using a u32 for identification, however, this will likely change once
	// we do some form of user authentication.
	pub id: u32,
	/// Defines how the user wants to receive their notifications.
	pub notifier: Notifier,
	/// Notifications the user enabled.
	// pub enabled_notifications: Vec<Notifications>,
	// The user's email, will be used if notifier != `Notifier::Telegram`
	pub email: String,
	// The user's telegram handle, used if tg_handle != `Notifier::Email`
	pub handle: String,
}

#[post("/register_user", data = "<registration_data>")]
pub async fn register_user(registration_data: Json<RegistrationData>) -> Result<(), &'static str> {
	// TODO: Check if the user is already registered. If they are, return an error.
	// Otherwise, register the new user.
	let conn = &User::get_connection().expect("DB connection not established");
	
	// check if user exists
	let user = User::query_by_id(conn, registration_data.id);
	assert!(user.is_err(), "User already exists");

	// Register user
	let user = User {
		id: 0,
		email: registration_data.email.clone(),
		tg_handle: registration_data.handle.clone(),
		notifier: registration_data.notifier.clone(),
	};
	User::create_user(conn, &user).expect("FAILED: User should created");
	
	Ok(())
}


