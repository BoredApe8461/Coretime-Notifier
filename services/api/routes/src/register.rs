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
	pub notifier: Option<Notifier>,
	// The user's email, will be used if notifier != `Notifier::Telegram`
	// #[validate(email)]
	pub email: String,
	// The user's telegram handle, used if tg_handle != `Notifier::Email`
	// #[validate]
	#[serde(rename = "tgHandle")]
	pub tg_handle: String,
	// Notifications the user enabled.
	// pub enabled_notifications: Vec<Notifications>,
}

impl Validate for RegistrationData {
	fn validate(&self) -> Result<(), ValidationErrors> {
		let mut errors = ValidationErrors::new();

		// check that at least an email and Telegram exists
		if self.email.len() == 0 && self.tg_handle.len() == 0 {
			let error = ValidationError::new("Email and Telegram cannot be empty");
			errors.add("email", error.clone());
			errors.add("tg_handle", error);
			return Err(errors);
		}

		// check that the passed value matches the notifier
		let result: Result<(), ValidationErrors> = match &self.notifier {
			Some(val) if val == &Notifier::Email => {
				// email must not be empty
				if self.email.len() == 0 { 
					errors.add("email", ValidationError::new("Email must not be empty"));
					return Err(errors) 
				} else {
					return Ok(())
				}
			},
			Some(val) if val == &Notifier::Telegram => {
				if self.tg_handle.len() == 0 {
					errors.add("tg_handle", ValidationError::new("Telegram handle must exist")); 
					return Err(errors) 
				} else {
					return Ok(())
				}
			}
			_ => Ok(())
		};

		result
	}
}

#[post("/register_user", data = "<registration_data>")]
pub async fn register_user(registration_data: Json<RegistrationData>) -> Result<status::Custom<()>, status::Custom<Json<ErrorResponse>>> {
	// Otherwise, register the new user.
	let conn = &User::get_connection().expect("DB connection not established");
	let validation = registration_data.validate();

	match validation {
		Err(message) => Err(status::Custom(
			Status::BadRequest,
			Json(ErrorResponse {
				message: message.to_string()
			})
		)),
		Ok(_) => {
			// TODO: Query by email | tg_handle (depending on what the notifier value is)
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
	}
	
}


