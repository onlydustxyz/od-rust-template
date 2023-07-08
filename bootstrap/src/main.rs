extern crate rocket;

use std::sync::Arc;

use domain::service::user_service::UserService;
use postgres_adapter::{
	adapter::postgres_adapter::PostgresUserAdapter, client::postgres_client::PostgresClient,
};
use rest_api_adapter::api::user_api::UserApi;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	let postgres_client = PostgresClient {
		user: "postgres".to_string(),
		password: "Passw0rd".to_string(),
		host: "localhost".to_string(),
		port: "5432".to_string(),
		database: "od-rust-template".to_string(),
	};
	postgres_client.run_migrations();
	let user_storage_port = Arc::new(PostgresUserAdapter { postgres_client });
	let user_facade_port = Arc::new(UserService { user_storage_port });
	let user_api = UserApi {};
	user_api.build_api(rocket::build(), user_facade_port).launch().await?;
	Ok(())
}
