use std::sync::Arc;

use diesel::{RunQueryDsl, SelectableHelper};
use domain::{
	model::organization::Organization,
	port::output::organization_storage_port::OrganizationStoragePort,
};

use crate::{
	client::postgres_client::PostgresClient,
	entity::{organization_entity::OrganizationEntity, schema::organizations::table},
};

pub struct PostgresOrganizationAdapter {
	pub postgres_client: Arc<PostgresClient>,
}

impl OrganizationStoragePort for PostgresOrganizationAdapter {
	fn create_organization(&self, organization: Organization) -> Result<Organization, String> {
		println!("Saving organization for name {}", organization.name());
		let connection = &mut self.postgres_client.establish_connection();

		let organization_entity = OrganizationEntity {
			id: organization.id.to_string(),
			name: organization.name.to_string(),
			external_id: organization.external_id.to_string(),
		};

		diesel::insert_into(table)
			.values(&organization_entity)
			.returning(OrganizationEntity::as_returning())
			.get_result(connection)
			.expect("Error while saving user");

		Ok(organization)
	}
}
