mod common;

#[cfg(test)]
mod tests {
	use bootstrap::bootstrap;
	use fake::{Fake, Faker};
	use postgres_adapter::config::postgres_configuration::PostgresConfiguration;
	use rest_api_adapter::dto::user_dto_response::UserDtoResponse;
	use rocket::{
		http::{ContentType, Status},
		local::blocking::Client,
		serde::json::json,
	};
	use testcontainers::{clients, clients::Cli, Container};

	use crate::common::integration_test_setup::IntegrationTestSetup;

	#[test]
	fn should_create_user() {
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

		let user_name = Faker.fake::<String>();
		let user_dto_request = json!({
			"name": user_name
		});

		// When
		let client = Client::tracked(rocket_builder).expect("valid rocket instance");
		let response = client
			.post("/api/v1/user")
			.body(user_dto_request.to_string())
			.header(ContentType::JSON)
			.dispatch();

		// Then
		assert_eq!(response.status(), Status::Ok);
		let user_dto_response: UserDtoResponse = response.into_json().unwrap();
		assert_eq!(user_name, user_dto_response.name);
	}
}
