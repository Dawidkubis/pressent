#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;
extern crate comrak;
extern crate serde;
extern crate structopt;
extern crate toml;

mod cli;
mod routes;
mod response;

use cli::Cli;
use std::env;
use structopt::StructOpt;

lazy_static! {
	pub static ref OPT: Cli = Cli::from_args();
}

fn main() {

	// port setting
	if let Some(i) = OPT.port {
		env::set_var("ROCKET_PORT", format!("{}", i));
	}

	// rocket server init
	rocket::ignite()
		.mount("/", routes![routes::index])
		.register(catchers![routes::not_found,])
		.launch();
}
