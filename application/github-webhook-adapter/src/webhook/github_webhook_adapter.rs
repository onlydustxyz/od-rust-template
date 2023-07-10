use std::sync::Arc;

use domain::port::input::organization_facade_port::OrganizationFacadePort;
use rocket::{
	http::Status,
	request::{FromRequest, Outcome},
	Build, Request, Rocket, State,
};

use crate::webhook::github_metadata::GithubEventMetadata;

pub struct GithubWebhookAdapter {}

impl GithubWebhookAdapter {
	pub fn build_webhook(
		&self,
		rocket_builder: Rocket<Build>,
		organization_facade_port: Arc<dyn OrganizationFacadePort + 'static>,
	) -> Rocket<Build> {
		#[post(
			"/github-app/webhook",
			format = "application/json",
			data = "<github_webhook_as_string>"
		)]
		async fn consume_webhook(
			github_event_metadata: GithubEventMetadata,
			organization_facade_port: &State<Arc<dyn OrganizationFacadePort>>,
			github_webhook_as_string: String,
		) {
			println!(
				"Consuming github webhook : {} with metadata github_event={} and github_signature={}",
				github_webhook_as_string,
				github_event_metadata.github_event_type,
				github_event_metadata.github_event_signature
			);
		}
		let _ = organization_facade_port
			.create_organization("name_test".to_string(), "external_id_test".to_string());

		rocket_builder
			.manage(organization_facade_port)
			.mount("/api/v1", routes![consume_webhook])
	}
}

#[cfg(test)]
mod tests {
	use std::{
		cell::{Cell, RefCell},
		sync::{Arc, RwLock},
	};

	use domain::{
		model::organization::Organization,
		port::input::{
			organization_facade_port,
			organization_facade_port::{MockOrganizationFacadePort, OrganizationFacadePort},
		},
	};
	use fake::{Fake, Faker};
	use mockall::{mock, predicate};
	use rocket::{
		http::{ContentType, Header, Status},
		local::blocking::Client,
		serde::json::json,
	};

	use crate::webhook::{
		github_metadata::{X_GITHUB_EVENT, X_GITHUB_SIGNATURE_256},
		github_webhook_adapter::GithubWebhookAdapter,
	};

	#[test]
	fn should_consume_github_installation_event() {
		// Given
		let github_event = Faker.fake::<String>();
		let rocket_builder = GithubWebhookAdapter {}
			.build_webhook(rocket::build(), Arc::new(OrganizationFacadeDummy {}));

		// When
		let client = Client::tracked(rocket_builder).expect("valid rocket instance");
		organization_facade_mock.expect_create_organization().times(1).with(
			predicate::eq("name_test".to_string()),
			predicate::eq("external_id_test".to_string()),
		);

		let response = client
			.post("/api/v1/github-app/webhook")
			.body(github_event)
			.header(Header::new(X_GITHUB_EVENT, "github_event_mock"))
			.header(Header::new(X_GITHUB_SIGNATURE_256, "installation"))
			.header(ContentType::JSON)
			.dispatch();

		// Then
		assert_eq!(response.status(), Status::Ok);
	}

	struct OrganizationFacadeDummy {}

	impl OrganizationFacadePort for OrganizationFacadeDummy {
		fn create_organization(
			&self,
			name: String,
			external_id: String,
		) -> Result<Organization, String> {
			todo!()
		}
	}
}
