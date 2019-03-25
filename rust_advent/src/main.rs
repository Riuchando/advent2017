// #[macro_use]
#![feature(test)]
extern crate chrono;
extern crate itertools;
use std::env;
use std::io::Error;

extern crate petgraph;
extern crate test;

use std::collections::HashSet;
use std::hash::Hash;

mod day7;
mod day8;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    match args[1].as_ref() {
        "day7" => {
            println!("{}", day7::part1()?);
            // println!("{}", day7::part2()?);
        }
        "day8" => {
            println!("{}", day8::part1()?);
            println!("{}", day8::part2()?);
        }
        _ => println!("{}", "invalid argument"),
    }
    Ok(())
}
