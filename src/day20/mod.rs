use std::collections::HashSet;

use itertools::Itertools;

type Pos = (i128, i128);
type Image = HashSet<Pos>;
type Algo = Vec<char>;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let (algo, mut image) = parse_input(&input);

    for i in 0..2 {
        image = evolve(&image, &algo, if i % 2 == 0 { '.' } else { '#' });
    }

    image.len() as i128
}

fn parse_input(input: &str) -> (Algo, Image) {
    let (algo, image) = input.split("\n\n")
        .tuples::<(&str, &str)>()
        .next()
        .unwrap();
    let algo: Algo = algo.chars().collect();

    let image: Image = image.split_whitespace()
        .enumerate()
        .flat_map(|(y, line)|
            line.chars().enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i128, y as i128))
        )
        .collect();

    (algo, image)
}

fn index(image: &Image, pos: Pos, default: char) -> usize {
    let (default, non_default) = if default == '#' {('1', '0')} else {('0', '1')};
    let (px, py) = pos;
    let mut bits = vec![];
    for y in (py - 1)..=(py + 1) {
        for x in (px - 1)..=(px + 1) {
            if image.contains(&(x, y)) {
                bits.push(non_default);
            } else {
                bits.push(default);
            }
        }
    }
    let bits: String = bits.iter().collect();
    usize::from_str_radix(&bits, 2).unwrap()
}

fn evolve(image: &Image, algo: &Algo, default: char) -> Image {
    let all_x: HashSet<_> = image.iter().map(|(x, _)| *x).collect();
    let all_y: HashSet<_> = image.iter().map(|(_, y)| *y).collect();

    let min_x = all_x.iter().min().unwrap();
    let max_x = all_x.iter().max().unwrap();
    let min_y = all_y.iter().min().unwrap();
    let max_y = all_y.iter().max().unwrap();

    let mut new_image = Image::new();
    for y in (min_y - 1)..=(max_y + 1) {
        for x in (min_x - 1)..=(max_x + 1) {
            let index = index(image, (x, y), default);
            let c = algo[index];

            if c == default {
                new_image.insert((x, y));
            }
        }
    } 

    new_image
}

fn star2(input: String) -> i128 {
    let (algo, mut image) = parse_input(&input);

    for i in 0..50 {
        image = evolve(&image, &algo, if i % 2 == 0 { '.' } else { '#' });
    }

    image.len() as i128
}
