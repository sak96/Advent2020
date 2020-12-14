pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;

use std::{
    env::args,
    fs::File,
    io::BufRead,
    io::{stdin, BufReader},
};

fn main() {
    let stdin = stdin();
    let reader: Box<dyn BufRead> = match args().nth(1) {
        Some(path) => Box::new(BufReader::new(File::open(path).unwrap())),
        None => Box::new(BufReader::new(stdin.lock())),
    };
    crate::day14::run(Box::new(reader));
}
