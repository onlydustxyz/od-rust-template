use diesel::prelude::*;

use crate::entity::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserEntity {
	pub id: String,
	pub name: String,
}
