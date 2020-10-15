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

		let options = ComrakOptions {
			unsafe_: true,
			..ComrakOptions::default()
		};

		let markdown = markdown_to_html(s, &options);

		format!(
			"
<!DOCTYPE html>
<html>
	<head>
		<script src=\"https://polyfill.io/v3/polyfill.min.js?features=es6\"></script>
		<script id=\"MathJax-script\" async src=\"https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js\"></script>
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
