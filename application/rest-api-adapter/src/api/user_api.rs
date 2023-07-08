use std::sync::Arc;

use domain::port::input::user_facade_port::UserFacadePort;
use rocket::{post, serde::json::Json, Build, Rocket, State};

use crate::dto::{user_dto_request::UserDtoRequest, user_dto_response::UserDtoResponse};

pub struct UserApi {}

impl UserApi {
	pub fn build_api(
		&self,
		rocket_builder: Rocket<Build>,
		user_facade_port: Arc<dyn UserFacadePort>,
	) -> Rocket<Build> {
		#[post("/user", format = "application/json", data = "<user_dto_request>")]
		async fn post_user(
			user_facade_port: &State<Arc<dyn UserFacadePort>>,
			user_dto_request: Json<UserDtoRequest>,
		) -> Json<UserDtoResponse> {
			println!("User name : {}", user_dto_request.name);

			user_facade_port
				.create_user(user_dto_request.name.to_string())
				.map(|user_created| {
					Json(UserDtoResponse {
						id: *user_created.id(),
						name: (*user_created.name).parse().unwrap(),
					})
				})
				.unwrap()
		}

		rocket_builder.manage(user_facade_port).mount("/api/v1", routes![post_user])
	}
}

#[cfg(test)]
mod tests {
	use std::sync::Arc;

	use domain::{model::user::User, port::input::user_facade_port::UserFacadePort};
	use fake::{Fake, Faker};
	use rocket::{
		http::{ContentType, Status},
		local::blocking::Client,
		serde::json::json,
	};

	use crate::{api::user_api::UserApi, rocket};

	#[test]
	fn should_post_create_user() {
		// Given
		let user_name = Faker.fake::<String>();
		let user_dto_request = json!({
			"name": user_name
		});
		let user_stub = User::create_user_from_name(user_name.clone());
		let user_id = user_stub.id;
		let mock = UserFacadeMock { user: user_stub };
		let rocket_builder = UserApi {}.build_api(rocket::build(), Arc::new(mock));

		// When
		let client = Client::tracked(rocket_builder).expect("valid rocket instance");
		let response = client
			.post("/api/v1/user")
			.body(user_dto_request.to_string())
			.header(ContentType::JSON)
			.dispatch();

		// Then
		assert_eq!(response.status(), Status::Ok);
		assert_eq!(
			format!("{{\"id\":\"{}\",\"name\":\"{}\"}}", user_id, user_name),
			response.into_string().unwrap()
		)
	}

	struct UserFacadeMock {
		user: User,
	}

	impl UserFacadePort for UserFacadeMock {
		fn create_user(&self, name: String) -> Result<User, String> {
			Ok(User {
				name: name.to_string(),
				id: self.user.id,
			})
		}
	}
}
