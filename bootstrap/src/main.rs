extern crate rocket;

use std::sync::Arc;

use domain::service::user_service::UserService;
use postgres_adapter::{
	adapter::postgres_adapter::PostgresUserAdapter, client::postgres_client::PostgresClient,
};
use rest_api_adapter::api::user_api::UserApi;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	PostgresClient {}.run_migrations();
	let user_storage_port = Arc::new(PostgresUserAdapter {});
	let user_facade_port = Arc::new(UserService { user_storage_port });
	let user_api = UserApi {};
	user_api.build_api(rocket::build(), user_facade_port).launch().await?;
	Ok(())
}
