use crate::response::MarkDown;
use crate::request::File;
use rocket::response::{NamedFile, Redirect};
use rocket::Request;
use anyhow::Result;
use std::path::PathBuf;

/// Index
#[get("/")]
pub fn index(s: File) -> MarkDown {
	let mut s = s.divide();
	let prefix = s.next();
	let r: String = s
		.enumerate()
		.map(|(x, _)| format!("\n## [{0}]({0})\n", x))
		.collect();
	MarkDown::new(&format!("{}\n{}", prefix.unwrap(), r))
}

#[get("/<num>")]
pub fn slide(num: usize, s: File) -> Option<MarkDown> {
	let mut s = s.divide();
	s.next();
	match s.nth(num) {
		Some(s) => Some(MarkDown::new(s)),
		None => None,
	}
}

/// any path
#[get("/<path..>", rank=2)]
pub fn path(path: PathBuf) -> Option<NamedFile> {
	NamedFile::open(path).ok()
}

/// 404 catcher
#[catch(404)]
pub fn not_found(_req: &Request) -> Redirect {
	Redirect::to(uri!(index))
}
