use bootstrap::bootstrap;
use postgres_adapter::config::postgres_configuration::PostgresConfiguration;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	bootstrap(PostgresConfiguration {
		user: "postgres".to_string(),
		password: "Passw0rd".to_string(),
		host: "localhost".to_string(),
		port: "5432".to_string(),
		database: "od-rust-template".to_string(),
	})
	.launch()
	.await?;
	Ok(())
}
