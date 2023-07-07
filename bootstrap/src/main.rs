extern crate rocket;

use rest_api_adapter::api::user_api::build_api;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	build_api().launch().await?;
	Ok(())
}
