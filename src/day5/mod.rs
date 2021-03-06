use std::cmp;
use std::collections::HashMap;
use std::iter;

use itertools::Itertools;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let lines: Vec<_> = input.split_whitespace()
        .tuples::<(&str, &str, &str)>()
        .map(|(a, _, b)| (v(a), v(b)))
        .collect();

    let mut map: HashMap<(i128, i128), i128> = HashMap::new();

    for ((x1, y1), (x2, y2)) in lines {
        let l = (cmp::max(i128::abs(x2 - x1), i128::abs(y2 - y1)) + 1) as usize;
        let positions: Vec<_> = if x1 == x2 {
            let x = iter::repeat(x1).take(l);
            let y = num::range_step_inclusive(y1, y2, num::signum(y2 - y1));
            x.zip(y).collect()
        } else if y1 == y2 {
            let x = num::range_step_inclusive(x1, x2, num::signum(x2 - x1));
            let y = iter::repeat(y1).take(l);
            x.zip(y).collect()
        } else {
            continue;
        };
        
        for &pos in positions.iter() {
            *map.entry(pos).or_insert(0) += 1
        }
    }

    map.values()
        .filter(|&&v| v >= 2)
        .count() as i128
}

fn v(input: &str) -> (i128, i128) {
    input.split(",")
        .map(|s| s.parse::<i128>().unwrap())
        .tuples::<(i128, i128)>()
        .next()
        .unwrap()
}

fn star2(input: String) -> i128 {
    let lines: Vec<_> = input.split_whitespace()
        .tuples::<(&str, &str, &str)>()
        .map(|(a, _, b)| (v(a), v(b)))
        .collect();

    let mut map: HashMap<(i128, i128), i128> = HashMap::new();

    for ((x1, y1), (x2, y2)) in lines {
        let l = (cmp::max(i128::abs(x2 - x1), i128::abs(y2 - y1)) + 1) as usize;
        let positions: Vec<_> = if x1 == x2 {
            let x = iter::repeat(x1).take(l);
            let y = num::range_step_inclusive(y1, y2, num::signum(y2 - y1));
            x.zip(y).collect()
        } else if y1 == y2 {
            let x = num::range_step_inclusive(x1, x2, num::signum(x2 - x1));
            let y = iter::repeat(y1).take(l);
            x.zip(y).collect()
        } else {
            let x = num::range_step_inclusive(x1, x2, num::signum(x2 - x1));
            let y = num::range_step_inclusive(y1, y2, num::signum(y2 - y1));
            x.zip(y).collect()
        };
        
        for &pos in positions.iter() {
            *map.entry(pos).or_insert(0) += 1
        }
    }

    map.values()
        .filter(|&&v| v >= 2)
        .count() as i128
}
