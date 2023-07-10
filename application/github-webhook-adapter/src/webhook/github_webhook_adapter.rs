use std::sync::Arc;

use domain::port::input::organization_facade_port::OrganizationFacadePort;
use rocket::{Build, Rocket};

use crate::webhook::github_metadata::GithubEventMetadata;

pub struct GithubWebhookAdapter {
	organization_facade_port: Arc<dyn OrganizationFacadePort>,
}

impl GithubWebhookAdapter {
	pub fn build(organization_facade_port: Arc<dyn OrganizationFacadePort>) -> Self {
		GithubWebhookAdapter {
			organization_facade_port,
		}
	}

	pub fn attach_webhook(&self, rocket_builder: Rocket<Build>) -> Rocket<Build> {
		#[post(
			"/github-app/webhook",
			format = "application/json",
			data = "<github_webhook_as_string>"
		)]
		async fn consume_webhook(
			github_event_metadata: GithubEventMetadata,
			github_webhook_as_string: String,
		) {
			println!(
				"Consuming github webhook : {} with metadata github_event={} and github_signature={}",
				github_webhook_as_string,
				github_event_metadata.github_event_type,
				github_event_metadata.github_event_signature
			);
		}
		let _ = &self
			.organization_facade_port
			.create_organization("name_test".to_string(), "external_id_test".to_string());

		rocket_builder.mount("/api/v1", routes![consume_webhook])
	}
}

#[cfg(test)]
mod tests {
	use std::sync::Arc;

	use domain::{
		model::organization::Organization,
		port::input::organization_facade_port::OrganizationFacadePort,
	};
	use fake::{Fake, Faker};
	use rocket::{
		http::{ContentType, Header, Status},
		local::blocking::Client,
	};

	use crate::webhook::{
		github_metadata::{X_GITHUB_EVENT, X_GITHUB_SIGNATURE_256},
		github_webhook_adapter::GithubWebhookAdapter,
	};

	#[test]
	fn should_consume_github_installation_event() {
		// Given
		let github_event = Faker.fake::<String>();
		let rocket_builder = GithubWebhookAdapter::build(Arc::new(OrganizationFacadeDummy {}))
			.attach_webhook(rocket::build());

		// When
		let client = Client::tracked(rocket_builder).expect("valid rocket instance");
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
			println!(
				"OrganizationFacadeDummy consuming name : {} and external_id {}",
				name, external_id
			);
			Ok(Organization::create_from_name_and_external_id(
				name,
				external_id,
			))
		}
	}
}
