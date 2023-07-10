use diesel::prelude::*;

use crate::entity::schema::organizations;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = organizations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrganizationEntity {
	pub id: String,
	pub name: String,
	pub external_id: String,
}
