extern crate rocket;

use std::sync::Arc;

use domain::service::{organization_service::OrganizationService, user_service::UserService};
use github_webhook_adapter::{self, webhook::github_webhook_adapter::GithubWebhookAdapter};
use postgres_adapter::{
	adapter::{
		postgres_organization_adapter::PostgresOrganizationAdapter,
		postgres_user_adapter::PostgresUserAdapter,
	},
	client::postgres_client::PostgresClient,
	config::postgres_configuration::PostgresConfiguration,
};
use rest_api_adapter::api::user_api::UserApi;
use rocket::{Build, Rocket};

pub fn bootstrap(postgres_configuration: PostgresConfiguration) -> Rocket<Build> {
	let postgres_client = Arc::new(PostgresClient::build(postgres_configuration));
	postgres_client.run_migrations();

	let user_storage_port = Arc::new(PostgresUserAdapter {
		postgres_client: postgres_client.clone(),
	});
	let user_facade_port = Arc::new(UserService { user_storage_port });
	let user_api = UserApi {};

	let organization_storage_port = Arc::new(PostgresOrganizationAdapter {
		postgres_client: postgres_client.clone(),
	});
	let organization_facade_port = Arc::new(OrganizationService {
		organization_storage_port,
	});
	let github_webhook_adapter = GithubWebhookAdapter {};

	let rocket_builder = rocket::build();
	let rocket_builder = user_api.build_api(rocket_builder, user_facade_port);
	github_webhook_adapter.build_webhook(rocket_builder, organization_facade_port)
}
