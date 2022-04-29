extern crate serde;
extern crate clap;
extern crate csv;

mod dataframe;
mod cmds;
mod app;

fn main() {
	match cmds::parse() {
		Ok(_) => {},
		Err(e) => panic!("{}", e),
	};
}

