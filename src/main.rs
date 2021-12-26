#[macro_use] extern crate maplit;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;

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

fn create_puzzle(day_number: u8, part_number: u8) -> Puzzle<String, i128> {
    match day_number {
        1 => Puzzle::new(part_number, day1::solver(part_number)),
        2 => Puzzle::new(part_number, day2::solver(part_number)),
        3 => Puzzle::new(part_number, day3::solver(part_number)),
        4 => Puzzle::new(part_number, day4::solver(part_number)),
        5 => Puzzle::new(part_number, day5::solver(part_number)),
        6 => Puzzle::new(part_number, day6::solver(part_number)),
        7 => Puzzle::new(part_number, day7::solver(part_number)),
        8 => Puzzle::new(part_number, day8::solver(part_number)),
        9 => Puzzle::new(part_number, day9::solver(part_number)),
        10 => Puzzle::new(part_number, day10::solver(part_number)),
        11 => Puzzle::new(part_number, day11::solver(part_number)),
        12 => Puzzle::new(part_number, day12::solver(part_number)),
        13 => Puzzle::new(part_number, day13::solver(part_number)),
        14 => Puzzle::new(part_number, day14::solver(part_number)),
        15 => Puzzle::new(part_number, day15::solver(part_number)),
        16 => Puzzle::new(part_number, day16::solver(part_number)),
        17 => Puzzle::new(part_number, day17::solver(part_number)),
        18 => Puzzle::new(part_number, day18::solver(part_number)),
        19 => Puzzle::new(part_number, day19::solver(part_number)),
        20 => Puzzle::new(part_number, day20::solver(part_number)),
        21 => Puzzle::new(part_number, day21::solver(part_number)),
        22 => Puzzle::new(part_number, day22::solver(part_number)),
        23 => Puzzle::new(part_number, day23::solver(part_number)),
        _ => panic!("Unknown day!"),
    }
}
