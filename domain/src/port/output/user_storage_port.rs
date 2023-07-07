use crate::model::user::User;

pub trait UserStoragePort {
	fn create_user(&self, user: User) -> Result<User, String>;
}
