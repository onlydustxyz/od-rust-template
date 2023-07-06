#[macro_use]
pub extern crate rocket;
use rocket::{get, Build, Rocket};

#[get("/")]
pub fn index() -> &'static str {
	"Hello, world!"
}

pub fn build_api() -> Rocket<Build> {
	rocket::build().mount("/", routes![index])
}
