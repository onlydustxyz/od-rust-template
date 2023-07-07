use std::sync::Arc;

use crate::{
	model::user::User,
	port::{input::user_facade_port::UserFacadePort, output::user_storage_port::UserStoragePort},
};

pub struct UserService {
	pub user_storage_port: Arc<dyn UserStoragePort>,
}

impl UserFacadePort for UserService {
	fn create_user(&self, name: String) -> Result<User, String> {
		let user = User::create_user_from_name(name);
		self.user_storage_port.create_user(user)
	}
}

#[cfg(test)]
mod tests {
	use std::sync::Arc;

	use crate::{
		model::user::User,
		port::{
			input::user_facade_port::UserFacadePort, output::user_storage_port::UserStoragePort,
		},
		service::user_service::UserService,
	};

	#[test]
	fn should_create_a_user_given_a_name() {
		// Given
		let user_storage_mock = UserStorageMock {};
		let user_storage_port = Arc::new(user_storage_mock);
		let user_service = UserService { user_storage_port };
		let user_name = "Pierre";

		// When
		let create_user_result = user_service.create_user(user_name.to_string());

		// Then
		assert!(create_user_result.is_ok());
		assert_eq!(user_name, create_user_result.unwrap().name())
	}

	struct UserStorageMock {}

	impl UserStoragePort for UserStorageMock {
		fn create_user(&self, user: User) -> Result<User, String> {
			Ok(user)
		}
	}
}
