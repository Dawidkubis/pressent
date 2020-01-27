use rocket::Request;
use crate::response::File;
use crate::OPT;
use std::path::PathBuf;

/// Index
#[get("/")]
pub fn index() -> File {
	File::open(&OPT.file).unwrap()
}

#[get("/<path..>")]
pub fn path(path: PathBuf) -> Option<File> {
	File::open(path).ok()
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
