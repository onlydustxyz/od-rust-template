use mockall::automock;

use crate::model::user::User;

#[automock]
pub trait UserFacadePort: Sync + Send {
	fn create_user(&self, name: String) -> Result<User, String>;
}
