use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GithubEventDto {
	pub action: String,
	pub installation: GithubInstallationDto,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GithubInstallationDto {
	pub id: i32,
	pub account: GithubAccountDto,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GithubAccountDto {
	pub id: i32,
	pub login: String,
}
