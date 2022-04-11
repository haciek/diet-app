extern crate serde;
extern crate csv as Csv;

use std::error::Error;
use std::process::exit;
mod dataframe;
mod csv;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { println!("Usage:\n{} print/input/modify", args[0]); exit(1);}

    match sel_option(args){
        Ok(_)   => println!("..done!"),
        Err(e)  => println!("{}", e),
    };

}

fn sel_option(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let opt: &str = &args[1];

    match opt {
        "print" => csv::print()?,
        "input" => csv::input(&args[2])?,
        "modify"=> csv::modify(&args[2], &args[3])?,
        _       => exit(1),
    };

    Ok(())
}
