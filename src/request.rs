use crate::OPT;
use anyhow::Error;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use std::fs::read_to_string;

/// a request guard for the input file
pub struct File(String);

impl File {
	pub fn divide(&self) -> impl Iterator<Item = &str> {
		self.0.split("\n<<>>\n")
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for File {
	type Error = Error;

	fn from_request(_request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		match read_to_string(&OPT.file) {
			Ok(s) => Outcome::Success(Self(s)),
			Err(e) => Outcome::Failure((Status::InternalServerError, anyhow!("{}", e))),
		}
	}
}
