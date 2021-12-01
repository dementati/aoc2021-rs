use std::env;
use std::fs;
use itertools::izip;

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&args[1])
        .expect("Couldn't read file");

    let depths: Vec<_> = contents.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let result = izip!(&depths, &depths[1..])
        .map(|(d1, d2)| d2 - d1)
        .filter(|d| d > &0)
        .count();

    println!("{}", result);
}
