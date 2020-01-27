use rocket::Request;
use crate::response::File;
use crate::OPT;

/// Index
#[get("/")]
pub fn index() -> File {
	File::open(&OPT.file).unwrap()
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
