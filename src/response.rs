use crate::MAIN;
use comrak::{markdown_to_html, ComrakOptions};
use rocket::request::Request;
use rocket::response::{self, content::Html, Responder};

pub struct MarkDown(String);

impl MarkDown {
	pub fn new(s: &str) -> Self {
		Self(Self::md(s))
	}

	fn md(s: &str) -> String {
		let css: String = MAIN.to_owned();

		let markdown = markdown_to_html(s, &ComrakOptions::default());

		format!(
				"
<!DOCTYPE html>
<html>
	<head>
		<style>{}</style>
		<section id=\"app\"></section>
		<script type=\"module\">
    import init from '/pkg/package.js';
    init('/pkg/package_bg.wasm');
		</script>
	</head>
	<body>
	{}
	</body>
</html>
				", css, &markdown
		)
	}
}

impl<'r> Responder<'r> for MarkDown {
	fn respond_to(self, r: &Request) -> response::Result<'r> {
		Html(self.0).respond_to(r)
	}
}
