use itertools::izip;

use crate::common;

pub fn solver(star: u8) -> fn(String) -> i32 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i32 {
    let depths = common::read_integers(&input);

    izip!(&depths, &depths[1..])
        .map(|(d1, d2)| d2 - d1)
        .filter(|d| d > &0)
        .count() as i32
}

fn star2(input: String) -> i32 {
    let depths = common::read_integers(&input);

    let sums: Vec<_> = izip!(&depths, &depths[1..], &depths[2..])
        .map(|(a, b, c)| a + b + c)
        .collect();

    izip!(&sums, &sums[1..])
        .map(|(d1, d2)| d2 - d1)
        .filter(|d| d > &0)
        .count() as i32
}
