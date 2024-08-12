use rocket::{post, serde::json::Json};
use serde::{Deserialize, Serialize};
use types::Notifications;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegistrationData {
	// TODO, for now we are using a u32 for identification, however, this will likely change once
	// we do some form of user authentication.
	pub id: u32,
	/// Defines how the user wants to receive their notifications.
	pub notifier: Notifier,
	/// Notifications the user enabled.
	pub enabled_notifications: Vec<Notifications>,
}

#[post("/register_user", data = "<registration_data>")]
pub async fn register_user(registration_data: Json<RegistrationData>) -> Result<(), &'static str> {
	// TODO: Check if the user is already registered. If they are, return an error.
	// Otherwise, register the new user.
	Ok(())
}
