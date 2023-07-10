use std::sync::Arc;

use crate::{
	model::organization::Organization,
	port::{
		input::organization_facade_port::OrganizationFacadePort,
		output::organization_storage_port::OrganizationStoragePort,
	},
};

struct OrganizationService {
	organization_storage_port: Arc<dyn OrganizationStoragePort>,
}

impl OrganizationFacadePort for OrganizationService {
	fn create_organization(&self, name: String, external_id: i32) -> Result<Organization, String> {
		self.organization_storage_port.create_organization(
			Organization::create_from_name_and_external_id(name, external_id),
		)
	}
}

#[cfg(test)]
mod tests {
	use std::sync::Arc;

	use fake::{Fake, Faker};

	use crate::{
		model::organization::Organization,
		port::{
			input::organization_facade_port::OrganizationFacadePort,
			output::organization_storage_port::OrganizationStoragePort,
		},
		service::organization_service::OrganizationService,
	};

	#[test]
	pub fn should_create_an_organization() {
		// Given
		let name = Faker.fake::<String>();
		let external_id = Faker.fake::<i32>();
		let organization_service = OrganizationService {
			organization_storage_port: Arc::new(OrganizationStorageMock {
				expected_name: name.clone(),
				expected_external_id: external_id.clone(),
			}),
		};

		// When
		let organization = organization_service.create_organization(name, external_id);

		// Then
		assert_eq!(true, organization.is_ok());
	}

	struct OrganizationStorageMock {
		expected_name: String,
		expected_external_id: i32,
	}

	impl OrganizationStoragePort for OrganizationStorageMock {
		fn create_organization(&self, organization: Organization) -> Result<Organization, String> {
			assert_eq!(
				self.expected_external_id.clone(),
				organization.external_id.to_owned()
			);
			assert_eq!(&self.expected_name, &organization.name.to_owned());
			Ok(organization)
		}
	}
}
