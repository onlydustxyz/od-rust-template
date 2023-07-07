use domain::{model::user::User, port::output::user_storage_port::UserStoragePort};

pub struct PostgresUserAdapter {}

impl UserStoragePort for PostgresUserAdapter {
	fn create_user(&self, user: User) -> Result<User, String> {
		println!("Saving user for name {}", user.name());
		Ok(user)
	}
}
