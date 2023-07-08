use diesel::{prelude::*, RunQueryDsl};
use domain::{model::user::User, port::output::user_storage_port::UserStoragePort};

use crate::{
	client::postgres_client::PostgresClient,
	entity::{schema::users::table, user_entity::UserEntity},
};

pub struct PostgresUserAdapter {
	pub postgres_client: PostgresClient,
}

impl UserStoragePort for PostgresUserAdapter {
	fn create_user(&self, user: User) -> Result<User, String> {
		println!("Saving user for name {}", user.name());
		let connection = &mut self.postgres_client.establish_connection();

		let user_entity = UserEntity {
			id: user.id.to_string(),
			name: user.name.to_string(),
		};

		diesel::insert_into(table)
			.values(&user_entity)
			.returning(UserEntity::as_returning())
			.get_result(connection)
			.expect("Error while saving user");

		Ok(user)
	}
}
