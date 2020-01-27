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

use cli::Cli;
use std::{env, thread, time};
use structopt::StructOpt;

fn main() {
	// get cmd args
	let opt = Cli::from_args();

	// port setting
	if let Some(i) = opt.port {
		env::set_var("ROCKET_PORT", format!("{}", i));
	}

	// rocket server init
	rocket::ignite()
		.mount("/", routes![routes::index])
		.register(catchers![routes::not_found,])
		.launch();
}
