use crate::response::MarkDown;
use crate::request::File;
use crate::OPT;
use rocket::response::NamedFile;
use rocket::Request;
use anyhow::Result;
use std::path::PathBuf;

/// Index
#[get("/")]
pub fn index(s: Result<File>) -> String {
	"Nigger".to_owned()
}

/// any path
#[get("/<path..>")]
pub fn path(path: PathBuf) -> Option<NamedFile> {
	NamedFile::open(path).ok()
}

/// 404 catcher
#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
