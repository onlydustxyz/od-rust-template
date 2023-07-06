extern crate rocket;

use rest_api_adapter::build_api;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	build_api().launch().await?;
	Ok(())
}
