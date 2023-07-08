use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::config::postgres_configuration::PostgresConfiguration;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct PostgresClient {
	pub postgres_configuration: PostgresConfiguration,
}

impl PostgresClient {
	pub fn establish_connection(&self) -> PgConnection {
		let database_url = format!(
			"postgres://{}:{}@{}:{}/{}",
			&self.postgres_configuration.user,
			&self.postgres_configuration.password,
			&self.postgres_configuration.host,
			&self.postgres_configuration.port,
			&self.postgres_configuration.database
		);

		PgConnection::establish(&database_url)
			.unwrap_or_else(|error| panic!("Error connecting to {} : {}", &database_url, error))
	}

	pub fn run_migrations(&self) {
		let connection = &mut self.establish_connection();

		connection.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");

		println!("Migrations successfully applied!");
	}
}
