use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Pos = (i128, i128);
type Image = HashMap<Pos, char>;
type Algo = Vec<char>;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let (algo, image) = parse_input(&input);

    println!("{:?}", algo);
    draw(&image);
    println!();
    let image = evolve(&image, &algo);
    draw(&image);
    println!();
    let image = evolve(&image, &algo);
    draw(&image);

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
    let (px, py) = pos;
    let mut bits = vec![];
    for y in (py - 1)..=(py + 1) {
        for x in (px - 1)..=(px + 1) {
            if image.contains_key(&(x, y)) {
                let bit = match image[&(x, y)] {
                    '#' => '1',
                    '.' => '0',
                    _ => panic!(),
                };
                bits.push(bit);
            } else {
                bits.push(default);
            }
        }
    }
    let bits: String = bits.iter().collect();
    usize::from_str_radix(&bits, 2).unwrap()
}

fn evolve(image: &Image, algo: &Algo) -> Image {
    let X: HashSet<_> = image.keys().map(|(x, _)| *x).collect();
    let Y: HashSet<_> = image.keys().map(|(_, y)| *y).collect();

    let min_x = X.iter().min().unwrap();
    let max_x = X.iter().max().unwrap();
    let min_y = Y.iter().min().unwrap();
    let max_y = Y.iter().max().unwrap();

    let mut new_image = Image::new();
    for y in (min_y - 1)..=(max_y + 1) {
        for x in (min_x - 1)..=(max_x + 1) {
            let index = index(image, (x, y));
            if algo[index] == '#' {
                new_image.insert((x, y));
            }
        }
    } 

    new_image
}

fn draw(image: &Image) {
    let X: HashSet<_> = image.iter().map(|(x, _)| *x).collect();
    let Y: HashSet<_> = image.iter().map(|(_, y)| *y).collect();

    let min_x = X.iter().min().unwrap();
    let max_x = X.iter().max().unwrap();
    let min_y = Y.iter().min().unwrap();
    let max_y = Y.iter().max().unwrap();

    for y in (min_y - 3)..=(max_y + 3) {
        for x in (min_x - 3)..=(max_x + 3) {
            print!("{}", if image.contains(&(x, y)) { '#' } else { '.'});
        }
        println!();
    } 
}

fn star2(input: String) -> i128 {
    0
}
