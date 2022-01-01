use chrono::{Datelike, Timelike, Utc};
use rand::Rng;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(_: String) -> i128 {
    fish_min();

    0
}

fn linear_search() {
    let mut digits = vec![8, 9, 9, 3, 7, 7, 9, 4, 9, 1, 9, 9, 3, 9];
    let mut count = 0;
    loop {
        increment(&mut digits);
        let z = compute_z(&digits);
        if z == 0 {
            display_time();
            println!("Solution: {:?}", digits);
            println!("{}", to_number(&digits));
        }
    }
}

fn linear_search_min() {
    let mut digits = vec![1, 7, 1, 1, 5, 5, 7, 4, 9, 1, 9, 1, 1, 2];
    let mut count = 0;
    loop {
        decrement(&mut digits);
        let z = compute_z(&digits);
        if z == 0 {
            display_time();
            println!("Solution: {:?}", digits);
            println!("{}", to_number(&digits));
        }
    }
}

fn display_time() {
    let now = Utc::now();

    println!(
        "{}-{:02}-{:02} {:?} {:02}:{:02}:{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.weekday(),
        now.hour(),
        now.minute(),
        now.second(),
    );
}

fn fish() {
    let mut best = 0;
    loop {
        let solution = find_solution();
        display_time();
        println!("Solution: {}", solution);
        if solution > best {
            best = solution;
        }
        println!("Current best: {}", best);
    }
}

fn fish_min() {
    let mut best = 9999_99999_99999;
    loop {
        let solution = find_solution();
        display_time();
        println!("Solution: {}", solution);
        if solution < best {
            best = solution;
        }
        println!("Current best: {}", best);
    }
}

fn find_solution() -> i128 {
    loop {
        let digits = generate();
        let (z, solution) = minimize(&digits);

        if z == 0 {
            return to_number(&solution);
        }
    }
}

fn to_number(number: &Vec<u8>) -> i128 {
    let s: String = number.iter()
        .map(|d| char::from_digit(*d as u32, 10).unwrap())
        .collect();

    s.parse().unwrap()
}

fn generate() -> Vec<u8> {
    let max = [1, 7, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9];
    let mut rng = rand::thread_rng();
    (0..14).map(|i| {
        rng.gen_range(1, max[i] + 1)
    }).collect()
}

fn minimize(digits: &Vec<u8>) -> (i128, Vec<u8>) {
    let mut prev_z = 0;
    let mut digits = digits.clone();
    for _ in 0..1000_000 {
        let (new_digits, z) = (1..digits.len())
            .map(|i| {
                let mut digits = digits.clone();
                if digits[i] > 1 {
                    digits[i] -= 1;
                } else {
                    digits[i] = 9;
                }
                let z = compute_z(&digits);
                (digits, z)
            })
            .min_by_key(|(_, z)| *z)
            .unwrap();

        digits = new_digits;

        if z == 0 || z == prev_z {
            return (z, digits);
        }

        prev_z = z;
    }

    (1, digits)
}

const A: [i128; 14] = [1, 1, 1, 1, 26, 1, 26, 1, 1, 26, 26, 26, 26, 26];
const B: [i128; 14] = [15, 15, 12, 13, -12, 10, -9, 14, 13, -14, -11, -2, -16, -14];
const C: [i128; 14] = [15, 10, 2, 16, 12, 11, 5, 16, 6, 15, 3, 12, 10, 13];

fn compute_z(number: &Vec<u8>) -> i128 {
    let mut z = 0;

    for i in 0..14 {
        let n = number[i] as i128;
        let x = if z % 26 + B[i] == n { 0 } else { 1 };
        z = (z / A[i]) * (25 * x + 1) + (n + C[i]) * x;
    }

    z
}

fn increment(number: &mut Vec<u8>) {
    for i in (0..14).rev() {
        if number[i] == 9 {
            number[i] = 1;
        } else {
            number[i] += 1;
            break;
        }
    }
}

fn decrement(number: &mut Vec<u8>) {
    for i in (0..14).rev() {
        if number[i] == 1 {
            number[i] = 9;
        } else {
            number[i] -= 1;
            break;
        }
    }
}

fn star2(input: String) -> i128 {
    linear_search_min();

    0
}