use diesel::prelude::*;

use crate::entity::schema::users;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserEntity {
	pub id: String,
	pub name: String,
}
