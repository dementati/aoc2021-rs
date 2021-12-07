use std::cmp;

use math::round;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    binary_search(&input, score)
}

fn binary_search(input: &str, score_fn: fn (&Vec<i128>, i128) -> i128) -> i128{
    let positions: Vec<i128> = input.split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut min = *positions.iter().min().unwrap();
    let mut max = *positions.iter().max().unwrap();

    loop {
        let pivot = min + div_up(max - min, 2);
        let a = min + div_down(pivot - min, 2);
        let b = pivot + div_up(max - pivot, 2);
        let v_a = score_fn(&positions, a);
        let v_b = score_fn(&positions, b);

        if v_a < v_b {
            max = pivot;
        } else {
            min = pivot;
        }

        if b - a == 1 {
            return cmp::min(v_a, v_b);
        }
    }
}

fn div_up(a: i128, b: i128) -> i128 {
    round::half_up(a as f64 / b as f64, 0) as i128
}

fn div_down(a: i128, b: i128) -> i128 {
    round::half_down(a as f64 / b as f64, 0) as i128
}

fn score(positions: &Vec<i128>, target: i128) -> i128 {
    positions.iter()
        .map(|c| (c - target).abs())
        .sum()
}

fn star2(input: String) -> i128 {
    binary_search(&input, score2)
}

fn score2(positions: &Vec<i128>, target: i128) -> i128 {
    positions.iter() 
        .map(|c| triangle((c - target).abs()))
        .sum()
}

fn triangle(n: i128) -> i128 {
    n * (n + 1) / 2
}
