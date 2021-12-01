use itertools::izip;

use crate::common;

pub fn solver(star: u8) -> fn(String) -> usize {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> usize {
    0
}

fn star2(input: String) -> usize {
    0
}
