use rocket::{http::Status, post, response::status, serde::json::Json};
use serde::{Deserialize, Serialize};
use types::{api::ErrorResponse, Notifier};
use validator::{Validate, ValidationError, ValidationErrors};

use storage::users::User;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegistrationData {
	// TODO, for now we are using a u32 for identification, however, this will likely change once
	// we do some form of user authentication.
	pub id: u32,
	// / Defines how the user wants to receive their notifications.
	pub notifier: Notifier,
	// The user's email, will be used if notifier != `Notifier::Telegram`
	// #[validate(email)]
	pub email: Option<String>,
	// The user's telegram handle, used if tg_handle != `Notifier::Email`
	// #[validate]
	#[serde(rename = "tgHandle")]
	pub tg_handle: Option<String>,
	// Notifications the user enabled.
	// pub enabled_notifications: Vec<Notifications>,
}

impl Validate for RegistrationData {
	fn validate(&self) -> Result<(), ValidationErrors> {
		let mut errors = ValidationErrors::new();

		// Ensure that the user has set either an email or Telegram handle.
		if self.email.is_none() && self.tg_handle.is_none() {
			let error = ValidationError::new("Email and Telegram cannot be empty");
			errors.add("email", error.clone());
			errors.add("tg_handle", error);
			return Err(errors);
		}

		// Ensure the configured notifier is set.
		match self.notifier {
			Notifier::Email =>
				if self.email.is_none() {
					errors.add("email", ValidationError::new("Email must not be empty"));
					return Err(errors)
				} else {
					Ok(())
				},
			Notifier::Telegram =>
				if self.tg_handle.is_none() {
					errors.add("tg_handle", ValidationError::new("Telegram handle must exist"));
					return Err(errors)
				} else {
					return Ok(())
				},
			_ => Ok(()),
		}
	}
}

#[post("/register_user", data = "<registration_data>")]
pub async fn register_user(
	registration_data: Json<RegistrationData>,
) -> Result<status::Custom<()>, status::Custom<Json<ErrorResponse>>> {
	// Otherwise, register the new user.
	let conn = &User::get_connection().expect("DB connection not established");

	registration_data.validate().map_err(|error| {
		status::Custom(Status::BadRequest, Json(ErrorResponse { message: error.to_string() }))
	})?;

	let error = Err(status::Custom(
		Status::BadRequest,
		Json(ErrorResponse { message: "User already exists".to_string() }),
	));
	if let Some(email) = registration_data.email.clone() {
		if User::query_by_email(conn, email).is_ok() {
			return error
		}
	}
	if let Some(tg_handle) = registration_data.tg_handle.clone() {
		if User::query_by_tg_handle(conn, tg_handle).is_ok() {
			return error
		}
	}

	// Register user
	let user = User {
		id: registration_data.id,
		email: registration_data.email.clone(),
		tg_handle: registration_data.tg_handle.clone(),
		notifier: registration_data.notifier.clone(),
	};

	User::create_user(conn, &user).map_err(|_| {
		status::Custom(
			Status::InternalServerError,
			Json(ErrorResponse { message: "Failed to register user".to_string() }),
		)
	})?;

	Ok(status::Custom(Status::Ok, ()))
}
