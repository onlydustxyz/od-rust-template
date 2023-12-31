use crate::model::organization::Organization;

pub trait OrganizationFacadePort: Sync + Send {
	fn create_organization(&self, name: String, external_id: i32) -> Result<Organization, String>;
}
