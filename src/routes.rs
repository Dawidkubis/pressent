use crate::response::MarkDown;
use crate::request::File;
use rocket::response::NamedFile;
use rocket::Request;
use anyhow::Result;
use std::path::PathBuf;

/// Index
#[get("/")]
pub fn index(s: Result<File>) -> MarkDown {
	let r: String = s.unwrap().divide()
		.enumerate()
		.map(|(x, _)| format!("\n## [{0}]({0})\n", x))
		.collect();
	MarkDown::new(&format!("# Index\n{}", r))
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
