use rocket::{http::Status, post, response::status, serde::json::Json};
use serde::{Deserialize, Serialize};
use types::{api::ErrorResponse, Notifications, Notifier};

use storage::users::User;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegistrationData {
	// TODO, for now we are using a u32 for identification, however, this will likely change once
	// we do some form of user authentication.
	pub id: u32,
	// / Defines how the user wants to receive their notifications.
	pub notifier: Option<Notifier>,
	// The user's email, will be used if notifier != `Notifier::Telegram`
	pub email: String,
	// The user's telegram handle, used if tg_handle != `Notifier::Email`
	pub tg_handle: String,
	// Notifications the user enabled.
	// pub enabled_notifications: Vec<Notifications>,
}

#[post("/register_user", data = "<registration_data>")]
pub async fn register_user(registration_data: Json<RegistrationData>) -> Result<status::Custom<()>, status::Custom<Json<ErrorResponse>>> {
	// Otherwise, register the new user.
	let conn = &User::get_connection().expect("DB connection not established");
	
	// check if user exists
	let user = User::query_by_id(conn, registration_data.id);
	if user.is_ok() {
		return Err(status::Custom(
			Status::BadRequest,
			Json(ErrorResponse {
				message: "User already exists".to_string()
			})
		))
	}

	let notifier = match registration_data.notifier.clone() {
		None => Notifier::Null,
		Some(val) => val
	};
	// Register user
	let user = User {
		id: 0,
		email: registration_data.email.clone(),
		tg_handle: registration_data.tg_handle.clone(),
		notifier,
	};
	let result = User::create_user(conn, &user);

	match result {
		Ok(_) => Ok(status::Custom(Status::Ok, ())),
		Err(_) => {
			return Err(status::Custom(
				Status::InternalServerError,
				Json(ErrorResponse {
					message: "Failed to register user".to_string(),
				}),
			));
		}
	}
}


