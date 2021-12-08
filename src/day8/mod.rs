use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    input.split("\n")
        .flat_map(|line| line.split_whitespace().rev().take(4))
        .filter(|s| [2, 3, 4, 7].contains(&s.len()) )
        .count() as i128
}

fn star2(input: String) -> i128 {
    input.split("\n")
        .map(|line| parse(line))
        .sum()
}

#[allow(dead_code)]
fn star2_2(input: &str) -> i128 {
    input.split("\n")
        .map(|line| {
            let (d, o) = line.split("|")
                .tuples::<(&str, &str)>()
                .next()
                .unwrap();

            let d: HashMap<usize, HashSet<char>> = d.split_whitespace()
                .map(|s| (s.len(), s.chars().collect()))
                .collect();

            o.split_whitespace()
                .map(|s| s.chars().collect::<HashSet<char>>())
                .map(|s| match (s.len(), s.intersection(&d[&4]).count(), s.intersection(&d[&2]).count()) {
                    (2, _, _) => 1,
                    (3, _, _) => 7,
                    (4, _, _) => 4,
                    (7, _, _) => 8,
                    (5, 2, _) => 2,
                    (5, 3, 1) => 5,
                    (5, 3, 2) => 3,
                    (6, 4, _) => 9,
                    (6, 3, 1) => 6,
                    (6, 3, 2) => 0,
                    _ => panic!("Oh no!"),
                })
                .fold(0, |acc, i| acc * 10 + i)
        })
        .sum()
}

fn parse(display: &str) -> i128 {
    let signals: Vec<HashSet<_>> = display.split_whitespace()
        .take(10)
        .map(|s| s.chars().collect())
        .collect();

    let mut digits: Vec<HashSet<char>> = vec![HashSet::new(); 10];

    digits[1] = extract(&signals, |s| s.len() == 2);
    digits[4] = extract(&signals, |s| s.len() == 4);
    digits[7] = extract(&signals, |s| s.len() == 3);
    digits[8] = extract(&signals, |s| s.len() == 7);
    digits[3] = extract(&signals, |s| s.len() == 5 && digits[1].is_subset(s));
    digits[9] = extract(&signals, |s| s.len() == 6 && digits[4].is_subset(s));

    let bl: HashSet<char> = digits[8].difference(&digits[9]).cloned().collect();

    digits[2] = extract(&signals, |s| s.len() == 5 && bl.is_subset(s));
    digits[5] = extract(&signals, |s| s.len() == 5 && !s.eq(&digits[2]) && !s.eq(&digits[3]));
    digits[6] = digits[5].union(&bl).cloned().collect();
    digits[0] = extract(&signals, |s| !digits.contains(s));

    display.split_whitespace().rev().take(4)
        .map(|s| s.chars().collect::<HashSet<char>>())
        .map(|s| digits.iter().position(|d| d.eq(&s)).unwrap())
        .collect::<Vec<usize>>().iter().rev()
        .fold(0, |acc, i| acc * 10 + i) as i128
}

fn extract<F>(signals: &Vec<HashSet<char>>, pred: F) -> HashSet<char> where F: Fn(&HashSet<char>) -> bool {
    signals.iter()
        .filter(|s| pred(s))
        .next()
        .unwrap()
        .clone()
}
