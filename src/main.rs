extern crate serde;
extern crate csv;

use std::process::exit;
mod dataframe;
mod app;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { println!("Usage:\n{} print/input/modify", args[0]); exit(1);}

    match app::sel_option(args){
        Ok(_)   => println!("..done!"),
        Err(e)  => println!("{}", e),
    };

}

