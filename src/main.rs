#![allow(non_snake_case)]
use std::io::prelude::*;
use std::{env, io};

mod day_01;
mod day_02;
mod day_03;

fn main() {
    let challenges = [day_01::main, day_02::main, day_03::main];

    let mut choice: usize;

    let mut args = env::args();
    if args.len() > 0 {
        choice = args.nth(1).unwrap().trim().parse().unwrap();
    } else {
        println!("Choose day to run (1-{})", challenges.len());

        let stdin = io::stdin();
        let mut buf = String::new();
        stdin
            .lock()
            .read_line(&mut buf)
            .expect("Failed on read_line");

        choice = buf.trim().parse().unwrap();
    }

    choice = (if choice == 0 {
        1
    } else {
        choice.min(challenges.len())
    }) - 1;
    println!("Running challenges from day {}!", choice + 1);
    challenges[choice]();
}
