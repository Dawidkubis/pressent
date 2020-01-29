use crate::request::File;
use crate::response::MarkDown;
use rocket::response::{NamedFile, Redirect};
use rocket::Request;
use std::path::PathBuf;

/// Index
#[get("/")]
pub fn index(s: File) -> MarkDown {
	let mut s = s.divide();
	let prefix = s.next();
	let r: String = s
		.enumerate()
		.map(|(x, _)| format!("\n## [{0}]({0})\n", x + 1))
		.collect();
	MarkDown::new(&format!("{}\n{}", prefix.unwrap(), r))
}

/// Slide by number
#[get("/<num>")]
pub fn slide(num: usize, s: File) -> Option<MarkDown> {
	let mut s = s.divide();
	s.next();
	
	if num == 0 {
		return None
	}

	match s.nth(num - 1) {
		Some(s) => Some(MarkDown::new(s)),
		None => None,
	}
}

/// any path
#[get("/<path..>", rank = 2)]
pub fn path(path: PathBuf) -> Option<NamedFile> {
	NamedFile::open(path).ok()
}

/// 404 catcher
#[catch(404)]
pub fn not_found(_req: &Request) -> Redirect {
	Redirect::to(uri!(index))
}
