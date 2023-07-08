use std::collections::HashMap;

use testcontainers::{core::WaitFor, Image};

const NAME: &str = "postgres";
const TAG: &str = "13-alpine";

#[derive(Debug)]
pub struct PostgresImage {
	pub env_vars: HashMap<String, String>,
}

impl Image for PostgresImage {
	type Args = ();

	fn name(&self) -> String {
		NAME.to_owned()
	}

	fn tag(&self) -> String {
		TAG.to_owned()
	}

	fn ready_conditions(&self) -> Vec<WaitFor> {
		vec![WaitFor::message_on_stderr(
			"database system is ready to accept connections",
		)]
	}

	fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
		Box::new(self.env_vars.iter())
	}
}
