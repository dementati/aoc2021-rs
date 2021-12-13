use std::collections::HashSet;

use itertools::Itertools;

type Pos = (i128, i128);
type Fold = (char, i128);

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let (dots, folds) = parse_input(&input);
    fold(dots, folds[0]).len() as i128
}

fn parse_input(input: &str) -> (HashSet<Pos>, Vec<Fold>) {
    let (dots, folds) = input.split("\n\n")
        .tuples::<(&str, &str)>()
        .next()
        .unwrap();

    let dots: HashSet<Pos> = dots.split_whitespace()
        .map(|line| line.split(",").tuples::<(&str, &str)>().next().unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let folds: Vec<Fold> = folds.split_whitespace()
        .tuples::<(&str, &str, &str)>()
        .map(|(_, _, s)| s.split("=").tuples::<(&str, &str)>().next().unwrap())
        .map(|(c, v)| (c.chars().next().unwrap(), v.parse().unwrap()))
        .collect();

    (dots, folds)
}

fn fold(dots: HashSet<Pos>, fold: Fold) -> HashSet<Pos> {
    let (c, v) = fold;
    dots.iter()
        .map(|(x, y)| 
            match c {
                'x' => (if x > &v { 2*v - x } else { *x }, *y),
                'y' => (*x, if y > &v { 2*v - y } else { *y }),
                _ => unreachable!(),
            }
        )
        .collect()
}

fn star2(input: String) -> i128 {
    let (mut dots, folds) = parse_input(&input);

    for f in folds.iter() {
        dots = fold(dots, *f);
    }

    let max_y = *dots.iter()
        .map(|(_, y)| y)
        .max()
        .unwrap();

    let max_x = *dots.iter()
        .map(|(x, _)| x)
        .max()
        .unwrap();

    println!();
    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{}", if dots.contains(&(x, y)) { "#" } else { "." });
        }
        println!();
    }

    0
}
