/// ## Coretime Notification Api 
///
/// This service exposes a web api though which coretime notifications can be managed.
///
/// Users will configure coretime notifications through a frontend interface. The frontend will then 
/// send these configurations to the web server exposed by this service for processing and storage.

use rocket_cors::CorsOptions;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
	rocket::build()
		.attach(CorsOptions::default().to_cors().unwrap())
		.mount("/", routes![])
}

// There should be three paths: one POST path to set the notification configuration, 
// one to update the configuration, and one to read the configuration of a user.
