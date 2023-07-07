use uuid::Uuid;

pub struct User {
	id: Uuid,
	name: String,
}

impl User {
	fn create_user_from_name(name: String) -> User {
		User {
			id: Uuid::new_v4(),
			name,
		}
	}
}
