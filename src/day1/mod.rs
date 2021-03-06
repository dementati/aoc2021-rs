use itertools::izip;

use crate::common;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let depths = common::read_integers(&input).unwrap();

    izip!(&depths, &depths[1..])
        .filter(|(d1, d2)| d1 < d2)
        .count() as _
}

fn star2(input: String) -> i128 {
    let depths = common::read_integers(&input).unwrap();

    izip!(&depths, &depths[3..])
        .filter(|(d1, d2)| d1 < d2)
        .count() as _
}
