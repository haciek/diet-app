use crate::clap::{Parser, Subcommand};


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
	/// Displays data and summary
	All,
	/// Displays data
	Data,
	/// Displays a summary of the data
	Summary,
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

