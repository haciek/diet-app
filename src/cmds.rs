use std::error::Error;
use crate::app;

pub fn parse() -> Result<(), Box<dyn Error >> {
    let cmd = clap::Command::new("weight_cli")
        .subcommand_required(true)
        .subcommands(
            vec![
                clap::Command::new("print"),
                clap::Command::new("input"),
                clap::Command::new("modify"),
                clap::Command::new("delete"),
            ]);

    let matches = cmd.get_matches();
    let option = match matches.subcommand() {
        Some(("print",  _)) => "print",
        Some(("input",  _)) => "input",
        Some(("modify", _)) => "modify",
        Some(("delete", _)) => "delete",
        _ => unreachable!("Invalid option"),
    };

    match option {
        "print" => app::print()?,
        "input" => app::input()?,
        "modify"=> app::modify()?,
        "delete"=> app::delete()?,
        _       => println!("Invalid option"),
    };

    Ok(())

}



