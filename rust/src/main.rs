extern crate num;
extern crate clap;

use clap::{Arg, App};

mod ntheory;
mod p11;
mod p23;
mod p33;
mod p57;
mod p65;
mod p119;

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
            "23" => p23::main(),
            "33" => p33::main(),
            "57" => p57::main(),
            "65" => p65::main(),
            "119" => p119::main(),
            _ => println!("Unknown problem id!"),
        }
    } else {
        p119::main(); // Run most recent problem.
    }

}
