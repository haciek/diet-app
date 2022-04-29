use crate::clap::Parser;
use std::error::Error;

use crate::app;
use crate::dataframe::{Args, Options};

pub fn parse() -> Result<(), Box<dyn Error>> {
	let args = Args::parse();

	match args.option {
		Options::Print {} => app::print()?,
		Options::Input { weight } => app::input(weight)?,
		Options::Modify { id, weight } => app::modify(id, weight)?,
		Options::Delete { id } => app::delete(id)?,
		_ => eprintln!("Invalid option"),
	};

	Ok(())
}
