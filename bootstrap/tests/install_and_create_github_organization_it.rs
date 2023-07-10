mod common;

#[cfg(test)]
mod tests {
	use std::{env, fs::File, io::Read};

	use bootstrap::bootstrap;
	use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
	use github_webhook_adapter::webhook::github_metadata::{
		X_GITHUB_EVENT, X_GITHUB_SIGNATURE_256,
	};
	use postgres_adapter::{
		client::postgres_client::PostgresClient,
		config::postgres_configuration::PostgresConfiguration,
		entity::{organization_entity::OrganizationEntity, schema::organizations::dsl::*},
	};
	use rocket::{
		http::{ContentType, Header, Status},
		local::blocking::Client,
	};
	use testcontainers::clients::Cli;

	use crate::common::integration_test_setup::IntegrationTestSetup;

	#[test]
	fn should_consume_github_event_and_create_linked_organization() {
		// Given
		let image = IntegrationTestSetup::init();
		let client = Cli::docker();
		let postgres_container = client.run(image);
		let postgres_port = postgres_container.ports().map_to_host_port_ipv4(5432).unwrap();
		let rocket_builder = bootstrap(PostgresConfiguration {
			user: "postgres".to_string(),
			password: "Passw0rd".to_string(),
			host: "localhost".to_string(),
			port: format!("{}", postgres_port).to_string(),
			database: "od-rust-template".to_string(),
		});
		let path = format!(
			"{}/resources/post_installation_creation.json",
			env::current_dir().unwrap().into_os_string().into_string().unwrap()
		);

		let mut file = File::open(path).unwrap();
		let mut github_event_as_string = String::new();
		file.read_to_string(&mut github_event_as_string).unwrap();

		// When
		let client = Client::tracked(rocket_builder).expect("valid rocket instance");
		let response = client
			.post("/api/v1/github-app/webhook")
			.body(github_event_as_string.to_string())
			.header(ContentType::JSON)
			.header(Header::new(X_GITHUB_EVENT, "github_event_mock"))
			.header(Header::new(X_GITHUB_SIGNATURE_256, "installation"))
			.dispatch();

		assert_eq!(response.status(), Status::Ok);
		let postgres_client = PostgresClient::build(PostgresConfiguration {
			user: "postgres".to_string(),
			password: "Passw0rd".to_string(),
			host: "localhost".to_string(),
			port: format!("{}", postgres_port).to_string(),
			database: "od-rust-template".to_string(),
		});
		let connection = &mut postgres_client.establish_connection();

		let results = organizations
			.select(OrganizationEntity::as_select())
			.load(connection)
			.expect("Error loading posts");
		assert_eq!(1, results.len());
	}
}
