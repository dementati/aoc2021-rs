use itertools::izip;

use crate::common;

pub fn day1(star: &str, filename: &str) {
    match star {
        "1" => star1(filename),
        "2" => star2(filename),
        _ => println!("Unknown star!"),
    }
}

fn star1(file: &str) {
    let depths = common::read_integers(file);

    let result = izip!(&depths, &depths[1..])
        .map(|(d1, d2)| d2 - d1)
        .filter(|d| d > &0)
        .count();

    println!("{}", result);
}

fn star2(file: &str) {
    let depths = common::read_integers(file);

    let sums: Vec<_> = izip!(&depths, &depths[1..], &depths[2..])
        .map(|(a, b, c)| a + b + c)
        .collect();

    let result = izip!(&sums, &sums[1..])
        .map(|(d1, d2)| d2 - d1)
        .filter(|d| d > &0)
        .count();

    println!("{}", result);
}
