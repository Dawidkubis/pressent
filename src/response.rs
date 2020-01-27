use crate::MAIN;
use anyhow::Result;
use comrak::{markdown_to_html, ComrakOptions};
use rocket::request::Request;
use rocket::response::{self, content::Html, Responder};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::Path;

pub struct MarkDown(Html<String>);

impl MarkDown {
	pub fn open<P>(path: P) -> Result<Self>
	where
		P: AsRef<OsStr> + AsRef<Path> + Debug,
	{
		Ok(Self(Html(Self::md(&read_to_string(path)?)?)))
	}

	pub fn md(body: &str) -> Result<String> {
		let skeleton: String = MAIN.to_owned();

		let markdown = markdown_to_html(body, &ComrakOptions::default());

		let markdown: String = markdown.split("<hr />")
			.map(|x| format!("<div class=\"slide\">{}</div>", x))
			.collect();

		Ok(skeleton.replace("{}", &markdown))
	}
}

impl<'r> Responder<'r> for MarkDown {
	fn respond_to(self, r: &Request) -> response::Result<'r> {
		self.0.respond_to(r)
	}
}
