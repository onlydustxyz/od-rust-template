use crate::dto::user_dto_request::UserDtoRequest;
use rocket::serde::json::Json;
use rocket::{post, Build, Rocket};

#[post("/user", format = "application/json", data = "<user_dto_request>")]
async fn post_user(user_dto_request: Json<UserDtoRequest>) {
	println!("User name : {}", user_dto_request.name)
}

pub fn build_api() -> Rocket<Build> {
	rocket::build().mount("/api/v1/", routes![post_user])
}
