use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserDtoResponse {
	pub id: Uuid,
	pub name: String,
}
