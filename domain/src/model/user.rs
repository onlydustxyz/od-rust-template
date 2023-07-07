use derive_getters::Getters;
use uuid::Uuid;

#[derive(Getters)]
pub struct User {
	pub id: Uuid,
	pub name: String,
}

impl User {
	pub fn create_user_from_name(name: String) -> User {
		User {
			id: Uuid::new_v4(),
			name,
		}
	}
}
