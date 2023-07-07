use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserDtoResponse {
	pub id: Uuid,
	pub name: String,
}
