extern crate anyhow;

use std::process::Command;
use std::env;

static FRONTEND:&str = "frontend";

fn main() -> anyhow::Result<()> {
	let p = env::var("PROFILE")?;
	if "release" == p.as_str() {
		Command::new("make").current_dir(FRONTEND).spawn()?;
	} else {
		Command::new("make")
			.arg("dev")
			.current_dir(FRONTEND)
			.spawn()?;
	}

	Ok(())
}
