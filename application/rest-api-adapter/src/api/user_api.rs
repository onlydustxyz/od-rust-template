use std::sync::Arc;

use domain::port::input::user_facade_port::UserFacadePort;
use rocket::{post, serde::json::Json, Build, Rocket, State};

use crate::dto::user_dto_request::UserDtoRequest;

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
		) {
			println!("User name : {}", user_dto_request.name);
			let _ = user_facade_port.create_user("test".to_string());
		}

		rocket_builder.manage(user_facade_port).mount("/api/v1/", routes![post_user])
	}
}
