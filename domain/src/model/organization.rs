use derive_getters::Getters;
use uuid::Uuid;

#[derive(Getters)]
pub struct Organization {
	pub id: Uuid,
	pub name: String,
	pub external_id: i32,
}

impl Organization {
	pub fn create_from_name_and_external_id(name: String, external_id: i32) -> Organization {
		Organization {
			id: Uuid::new_v4(),
			name,
			external_id,
		}
	}
}
