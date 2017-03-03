extern crate clap;
use clap::{Arg, App};

mod p11;

fn main() {
    let matches = App::new("Rust-Euler")
        .version("42.0")
        .author("Lukas K.")
        .about("Solves Project Euler solutions in Rust.")
        .arg(Arg::with_name("problem")
            .short("p")
            .long("problem")
            .takes_value(true)
            .required(false))
        .get_matches();

    if let Some(problem) = matches.value_of("problem") {
        match problem {
            "11" => p11::main(),
            _ => println!("Unknown problem id!"),
        }
    } else {
        p11::main();
        // Run most recent problem.
    }
    
}
