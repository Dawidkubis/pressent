use rocket::Request;
use crate::response::File;

/// Index
#[get("/")]
pub fn index() -> File {
	File::open("index.html").unwrap()
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
	format!("404: {} is not a valid path", req.uri())
}
