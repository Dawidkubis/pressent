use crate::response::MarkDown;
use crate::OPT;
use rocket::response::NamedFile;
use rocket::Request;
use std::path::PathBuf;

/// Index
#[get("/")]
pub fn index() -> MarkDown {
	MarkDown::open(&OPT.file).unwrap()
}

#[get("/<num>")]
pub fn slide(num: u64) -> MarkDown {
	MarkDown::open(&OPT.file)
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
