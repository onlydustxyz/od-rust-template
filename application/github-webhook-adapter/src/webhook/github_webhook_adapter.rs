use std::sync::Arc;

use domain::port::input::organization_facade_port::OrganizationFacadePort;
use rocket::{Build, Rocket, State};

use crate::webhook::{github_event_dto::GithubEventDto, github_metadata::GithubEventMetadata};

pub struct GithubWebhookAdapter {}

impl GithubWebhookAdapter {
	pub fn build_webhook(
		&self,
		rocket_builder: Rocket<Build>,
		organization_facade_port: Arc<dyn OrganizationFacadePort>,
	) -> Rocket<Build> {
		#[post(
			"/github-app/webhook",
			format = "application/json",
			data = "<github_webhook_as_string>"
		)]
		async fn consume_webhook(
			github_event_metadata: GithubEventMetadata,
			github_webhook_as_string: String,
			organization_facade_port: &State<Arc<dyn OrganizationFacadePort>>,
		) {
			println!(
				"Consuming github webhook : {} with metadata github_event={} and github_signature={}",
				github_webhook_as_string,
				github_event_metadata.github_event_type,
				github_event_metadata.github_event_signature
			);

			let github_event_dto: GithubEventDto =
				serde_json::from_str(&github_webhook_as_string).unwrap();
			let _ = organization_facade_port.create_organization(
				github_event_dto.installation.account.login.to_string(),
				github_event_dto.installation.account.id.to_owned(),
			);
		}

		rocket_builder
			.manage(organization_facade_port)
			.mount("/api/v1", routes![consume_webhook])
	}

	fn handle_github_event(&self, github_event_as_string: String) {
		let github_event_dto: GithubEventDto =
			serde_json::from_str(&github_event_as_string).unwrap();
		println!("{}", github_event_dto.action);
	}
}

#[cfg(test)]
mod tests {
	use std::{env, fs::File, io::Read, sync::Arc};

	use domain::{
		model::organization::Organization,
		port::input::organization_facade_port::OrganizationFacadePort,
	};
	use mockall::mock;
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
		let mut file = File::open(format!(
			"{}/resources/post_installation_creation.json",
			env::current_dir().unwrap().into_os_string().into_string().unwrap()
		))
		.unwrap();
		let mut github_event_as_string = String::new();
		file.read_to_string(&mut github_event_as_string).unwrap();
		let rocket_builder = GithubWebhookAdapter {}.build_webhook(
			rocket::build(),
			Arc::new(OrganizationFacadeDummy {
				expected_name: "Barbicane-fr".to_string(),
				expected_external_id: 58205251,
			}),
		);
		// When
		let client = Client::tracked(rocket_builder).expect("valid rocket instance");
		let response = client
			.post("/api/v1/github-app/webhook")
			.body(github_event_as_string)
			.header(Header::new(X_GITHUB_EVENT, "github_event_mock"))
			.header(Header::new(X_GITHUB_SIGNATURE_256, "installation"))
			.header(ContentType::JSON)
			.dispatch();

		// Then
		assert_eq!(response.status(), Status::Ok);
	}

	struct OrganizationFacadeDummy {
		expected_name: String,
		expected_external_id: i32,
	}

	impl OrganizationFacadePort for OrganizationFacadeDummy {
		fn create_organization(
			&self,
			name: String,
			external_id: i32,
		) -> Result<Organization, String> {
			println!(
				"OrganizationFacadeDummy consuming name : {} and external_id {}",
				name, external_id
			);
			assert_eq!(self.expected_name, name);
			assert_eq!(self.expected_external_id, external_id);
			Ok(Organization::create_from_name_and_external_id(
				name,
				external_id,
			))
		}
	}
}
