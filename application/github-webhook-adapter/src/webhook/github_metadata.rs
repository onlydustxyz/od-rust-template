use rocket::{
	http::Status,
	request::{FromRequest, Outcome},
	Build, Request, Rocket,
};

pub static X_GITHUB_EVENT: &str = "X-GitHub-Event";
pub static X_GITHUB_SIGNATURE_256: &str = "X-Hub-Signature-256";

pub struct GithubEventMetadata {
	pub github_event_type: String,
	pub github_event_signature: String,
}

#[async_trait]
impl<'r> FromRequest<'r> for GithubEventMetadata {
	type Error = ();

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let x_github_event = request.headers().get_one(X_GITHUB_EVENT);
		let x_github_signature_256 = request.headers().get_one(X_GITHUB_SIGNATURE_256);
		if x_github_event.is_some() && x_github_signature_256.is_some() {
			Outcome::Success(GithubEventMetadata {
				github_event_type: x_github_event.unwrap().to_string(),
				github_event_signature: x_github_signature_256.unwrap().to_string(),
			})
		} else {
			Outcome::Failure((Status::InternalServerError, ()))
		}
	}
}
