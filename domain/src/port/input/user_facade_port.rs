use crate::model::user::User;

pub trait UserFacadePort: Sync + Send {
	fn create_user(&self, name: String) -> Result<User, String>;
}
