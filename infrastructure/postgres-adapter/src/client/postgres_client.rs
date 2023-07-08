use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct PostgresClient {
	pub user: String,
	pub password: String,
	pub host: String,
	pub port: String,
	pub database: String,
}

impl PostgresClient {
	pub fn establish_connection(&self) -> PgConnection {
		let database_url = format!(
			"postgres://{}:{}@{}:{}/{}",
			&self.user, &self.password, &self.host, &self.port, &self.database
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
