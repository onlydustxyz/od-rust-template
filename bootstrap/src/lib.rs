extern crate rocket;

use std::sync::Arc;

use domain::service::user_service::UserService;
use postgres_adapter::{
	adapter::postgres_adapter::PostgresUserAdapter, client::postgres_client::PostgresClient,
	config::postgres_configuration::PostgresConfiguration,
};
use rest_api_adapter::api::user_api::UserApi;
use rocket::{Build, Rocket};

pub fn bootstrap(postgres_configuration: PostgresConfiguration) -> Rocket<Build> {
	let postgres_client = PostgresClient::build(postgres_configuration);
	postgres_client.run_migrations();
	let user_storage_port = Arc::new(PostgresUserAdapter { postgres_client });
	let user_facade_port = Arc::new(UserService { user_storage_port });
	let user_api = UserApi {};
	user_api.build_api(rocket::build(), user_facade_port)
}
