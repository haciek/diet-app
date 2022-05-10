extern crate serde;
extern crate clap;
extern crate csv;

mod dataframe;
mod args;
mod app;

fn main() {
	match args::parse() {
		Ok(_) => {},
		Err(e) => panic!("{}", e),
	};
}

