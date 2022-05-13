extern crate serde;
extern crate clap;
extern crate csv;

use crate::args::{Args, Options};
use crate::app::CsvData;
use std::error::Error;
use clap::Parser;

mod args;
mod app;

fn main() -> Result<(), Box<dyn Error>> {
	let args = Args::parse();
	let mut csv_data = CsvData::new()?;
	match args.option {
		Options::All {} => csv_data.show_all(),
		Options::Data {} => csv_data.show_data(),
		Options::Summary {} => csv_data.show_summary(),
		Options::Input { weight } => csv_data.append(weight)?,
		Options::Modify { id, weight } => csv_data.modify(id, weight)?,
		Options::Delete { id } => csv_data.delete(id)?,
	};

	Ok(())
}

