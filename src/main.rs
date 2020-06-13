#[macro_use]
extern crate lazy_static;

use clap::App;
mod ambuster;
mod expression;
mod lexer;
mod parser;

fn main() {
    let matches = App::new("sass-rta")
        .version("0.0.1")
        .about("Does awesome things")
        .arg("-d, --dry-run 'Only prints results'")
        .subcommand(
            App::new("write")
                .about("write markings")
                .arg("<target> 'Sets an optional target file'"),
        )
        .subcommand(
            App::new("reset")
                .about("reset markings")
                .arg("<target> 'Sets an optional target file'"),
        )
        .get_matches();

    if matches.is_present("dry-run") {
        println!("[dry run]");
    }

    if let Some(ref matches) = matches.subcommand_matches("write") {
        if let Some(o) = matches.value_of("target") {
            println!("write: Value for target: {}", o);
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("reset") {
        if let Some(o) = matches.value_of("target") {
            println!("reset: Value for target: {}", o);
        }
    }

    // Continued program logic goes here...
}
