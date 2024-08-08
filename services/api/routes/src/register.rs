
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegistrationData {}

#[post("/register_user", data = "<registration_data>")]
pub async fn register_user(registration_data: Json<RegistrationData>) -> Result<(), Error> {


	Ok(())
}
