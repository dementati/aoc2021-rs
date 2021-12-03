mod common;
mod day1;
mod day2;
mod day3;

use std::env;
use std::fs;

use aoc_helper::{AocDay, Puzzle};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_number = &args[1].parse::<u8>().unwrap();
    let part_number = &args[2].parse::<u8>().unwrap();
    let maybe_file = args.get(3);

    run(*day_number, *part_number, maybe_file);
}

fn run(day_number: u8, part_number: u8, maybe_file: Option<&String>) {
    let mut puzzle = create_puzzle(day_number, part_number);
    let mut day = AocDay::new(2021, day_number);

    if let Some(file) = maybe_file {
        let contents = fs::read_to_string(file)
            .expect("Can't read test file!");

        puzzle.examples(&[contents]);
        day.test(&puzzle);
    } else {
        day.run(&puzzle).expect("Failed to run!");
    }
}

fn create_puzzle(day_number: u8, part_number: u8) -> Puzzle<String, i32> {
    match day_number {
        1 => Puzzle::new(part_number, day1::solver(part_number)),
        2 => Puzzle::new(part_number, day2::solver(part_number)),
        3 => Puzzle::new(part_number, day3::solver(part_number)),
        _ => panic!("Unknown day!"),
    }
}
