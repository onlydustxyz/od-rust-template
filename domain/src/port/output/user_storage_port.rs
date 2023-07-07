use crate::model::user::User;

pub trait UserStoragePort: Sync + Send {
	fn create_user(&self, user: User) -> Result<User, String>;
}
