use diesel::{
	r2d2::{ConnectionManager, Pool, PooledConnection},
	PgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::config::postgres_configuration::PostgresConfiguration;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct PostgresClient {
	pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresClient {
	pub fn build(postgres_configuration: PostgresConfiguration) -> Self {
		let database_url = format!(
			"postgres://{}:{}@{}:{}/{}",
			postgres_configuration.user,
			postgres_configuration.password,
			postgres_configuration.host,
			postgres_configuration.port,
			postgres_configuration.database
		);
		let manager = ConnectionManager::<PgConnection>::new(database_url);
		PostgresClient {
			pool: Pool::builder().max_size(5).build(manager).unwrap(),
		}
	}

	pub fn establish_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
		self.pool.get().unwrap()
	}

	pub fn run_migrations(&self) {
		let connection = &mut self.establish_connection();
		connection.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");

		println!("Migrations successfully applied!");
	}
}
