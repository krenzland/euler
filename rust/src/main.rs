extern crate num;
extern crate clap;
extern crate bit_vec;

use clap::{Arg, App};

mod ntheory;
mod p11;
mod p23;
mod p27;
mod p30;
mod p33;
mod p34;
mod p35;
mod p36;
mod p56;
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
            "27" => p27::main(),
            "30" => p30::main(),
            "33" => p33::main(),
            "34" => p34::main(),
            "35" => p35::main(),
            "36" => p36::main(),
            "56" => p56::main(),
            "57" => p57::main(),
            "65" => p65::main(),
            "119" => p119::main(),
            _ => println!("Unknown problem id!"),
        }
    } else {
        p56::main(); // Run most recent problem.
    }

}
