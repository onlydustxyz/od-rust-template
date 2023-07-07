use crate::model::user::User;

pub trait UserFacadePort {
	fn create_user(&self, name: String) -> Result<User, String>;
}
