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
mod response;
mod routes;
mod request;

use cli::Cli;
use std::env;
use structopt::StructOpt;
//use anyhow::Result;

lazy_static! {
	pub static ref OPT: Cli = Cli::from_args();
}

pub static MAIN: &'static str = include_str!("main.css");

fn main() {
	// port setting
	if let Some(i) = OPT.port {
		env::set_var("ROCKET_PORT", format!("{}", i));
	}

	// rocket server init
	rocket::ignite()
		.mount("/", routes![routes::index, routes::path, routes::slide])
		.register(catchers![routes::not_found,])
		.launch();
}
