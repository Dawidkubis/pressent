#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;
extern crate comrak;
extern crate structopt;

mod cli;
mod request;
mod response;
mod routes;

use anyhow::Result;
use cli::Cli;
use std::env;
use std::path::Path;
use structopt::StructOpt;

lazy_static! {
	pub static ref OPT: Cli = Cli::from_args();
}

pub static MAIN: &str = include_str!("main.css");
pub static JS: &str = include_str!("main.js");

fn main() -> Result<()> {
	// check if file exists
	if !Path::new(&OPT.file).exists() {
		panic!("file not found: {}", OPT.file);
	}

	// port setting
	if let Some(i) = OPT.port {
		env::set_var("ROCKET_PORT", format!("{}", i));
	}

	// rocket server init
	rocket::ignite()
		.mount("/", routes![routes::index, routes::path, routes::slide])
		.register(catchers![routes::not_found,])
		.launch();

	Ok(())
}
