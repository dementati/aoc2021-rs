use num::{BigInt, Zero};

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let data = parse_input(&input);
    evolve(&data, 80)
}

fn star2(input: String) -> i128 {
    let data = parse_input(&input);
    evolve(&data, 256)
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

fn evolve(data: &Vec<i128>, max_gen: i128) -> i128 {
    let mut data = data.clone();
    let mut i0 = 0;
    for _ in 0..max_gen {
        let new = data[i0];
        i0 = (i0 + 1) % 9;
        data[(i0 + 8) % 9] = new;
        data[(i0 + 6) % 9] += new;
    }

    data.iter().sum()
}

fn evolve_big(data: &Vec<BigInt>, max_gen: i128) -> BigInt {
    let mut data = data.clone();
    let mut i0 = 0;
    for _ in 0..max_gen {
        let new = data[i0].clone();
        i0 = (i0 + 1) % 9;
        data[(i0 + 6) % 9] += &new;
        data[(i0 + 8) % 9] = new;
    }

    data.iter().sum()
}
