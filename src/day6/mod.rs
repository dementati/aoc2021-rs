use num::{BigInt, Zero};

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    solve(&input, 80)
}

fn star2(input: String) -> i128 {
    solve(&input, 256)
}

fn solve(input: &str, max_gen: usize) -> i128 {
    evolve(&parse_input(input), max_gen)
}

#[allow(dead_code)]
fn solve_big(input: &str, max_gen: usize) -> BigInt {
    evolve_big(&parse_input_big(&input), max_gen)
}

fn parse_input(input: &str) -> Vec<i128> {
    let input: Vec<i128> = input.split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut data = vec![0; 9];

    for &v in input.iter() {
        data[v as usize] += 1;
    }

    data
}

#[allow(dead_code)]
fn parse_input_big(input: &str) -> Vec<BigInt> {
    let input: Vec<u8> = input.split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut data = vec![Zero::zero(); 9];

    for &v in input.iter() {
        let i = v as usize;
        data[i] += 1;
    }

    data
}

fn evolve(data: &Vec<i128>, max_gen: usize) -> i128 {
    let mut data = data.clone();
    for i in 0..max_gen {
        data[(i + 7) % 9] += data[i % 9];
    }
    data.iter().sum()
}

#[allow(dead_code)]
fn evolve_big(data: &Vec<BigInt>, max_gen: usize) -> BigInt {
    let mut data = data.clone();
    for i in 0..max_gen {
        let new = data[i % 9].clone();
        data[(i + 7) % 9] += new;
    }
    data.iter().sum()
}
