use crate::model::organization::Organization;

pub trait OrganizationStoragePort: Send + Sync {
	fn create_organization(&self, organization: Organization) -> Result<Organization, String>;
}
