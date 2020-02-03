use crate::{JS, MAIN, OPT};
use comrak::{markdown_to_html, ComrakOptions};
use rocket::request::Request;
use rocket::response::{self, content::Html, Responder};

pub struct MarkDown(String);

/// markdown server-side compilation
impl MarkDown {
	pub fn new(s: &str) -> Self {
		Self(Self::md(s))
	}

	fn md(s: &str) -> String {
		let css: String = MAIN.to_owned();
		let js: String = JS.to_owned();

		let markdown = markdown_to_html(s, &ComrakOptions::default());

		format!(
			"
<!DOCTYPE html>
<html>
	<head>
		<style>{}</style>
		<script>{}</script>
		<title>{}</title>
	</head>
	<body>
	{}
	</body>
</html>
				",
			css, js, &OPT.file, &markdown
		)
	}
}

impl<'r> Responder<'r> for MarkDown {
	fn respond_to(self, r: &Request) -> response::Result<'r> {
		Html(self.0).respond_to(r)
	}
}
