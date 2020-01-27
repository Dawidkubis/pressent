use anyhow::Result;
use rocket::response::NamedFile;
use rocket::Request;
use std::path::{Path, PathBuf};

/// Index
#[get("/")]
pub fn index() -> NamedFile {
	NamedFile::open("index.html").unwrap()
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
