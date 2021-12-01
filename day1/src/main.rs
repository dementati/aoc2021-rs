use std::env;
use std::fs;
use itertools::izip;

fn main() {
    let args: Vec<String> = env::args().collect();
    let star = &args[1];
    
    match star.as_str() {
        "1" => day1(&args[2]),
        "2" => day2(&args[2]),
        _ => println!("Unknown star!"),
    }
}

fn day1(file: &str) {
    let contents = fs::read_to_string(file)
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

fn day2(file: &str) {
    let contents = fs::read_to_string(file)
        .expect("Couldn't read file");

    let depths: Vec<_> = contents.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let sums: Vec<_> = izip!(&depths, &depths[1..], &depths[2..])
        .map(|(a, b, c)| a + b + c)
        .collect();

    let result = izip!(&sums, &sums[1..])
        .map(|(d1, d2)| d2 - d1)
        .filter(|d| d > &0)
        .count();

    println!("{}", result);
}
