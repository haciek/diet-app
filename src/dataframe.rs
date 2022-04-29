use crate::clap::{Parser, Subcommand};
use crate::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
	pub id: u32,
	pub date: String,
	pub weight: f32,
}

#[derive(Parser)]
#[clap(
	author="Maciej Habasiński",
	version,
	about="CLI app for tracking weight over time using a csv file.",
	long_about=None)]

pub struct Args {
	#[clap(subcommand)]
	pub option: Options,
}

#[derive(Subcommand)]
pub enum Options {
	/// Prints existing data
	Print,
	/// Deletes a record
	Delete {
		#[clap(short, long)]
		id: u32,
	},
	/// Appends a new record
	Input {
		#[clap(short, long)]
		weight: f32,
	},
	/// Modifies a record
	Modify {
		#[clap(short, long)]
		id: u32,
		#[clap(short, long)]
		weight: f32,
	},
}
