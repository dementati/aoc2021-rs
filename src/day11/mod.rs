use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

type Pos = (i128, i128);
type Map = HashMap<Pos, i128>;
type Set = HashSet<Pos>;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let (mut map, _) = parse_input(&input);

    let mut total: i128 = 0;
    for _ in 0..100 {
        let (new_map, count) = evolve(map);
        map = new_map;
        total += count;
    }

    total
}

fn parse_input(input: &str) -> (Map, i128) {
    let dim = input.split_whitespace().count() as i128;
    let map = input.split_whitespace()
        .enumerate()
        .flat_map(|(y, line)|
            line.chars()
                .enumerate()
                .map(move |(x, c)| 
                    (
                        (x as i128, y as i128), 
                        c.to_digit(10).unwrap() as i128
                    )
                )
        )
        .collect();

    (map, dim)
}

fn evolve(map: Map) -> (Map, i128) {
    let mut map: Map = map.into_iter()
        .map(|(k, v)| (k, v + 1))
        .collect();

    let mut closed = Set::new();
    let mut count = 0;
    loop {
        let open = get_open(&map, &closed);
        if open.is_empty() {
            break;
        }

        count += open.len();
        for pos in open.iter() {
            let ns = neighbours(&map, *pos); 
            for n in ns {
                *map.get_mut(&n).unwrap() += 1;
            }
        }

        closed.extend(&open);
    }

    let map = map.iter()
        .map(|(pos, &v)| (*pos, if v > 9 { 0 } else { v }))
        .collect();

    (map, count as i128)
}

fn neighbours(map: &Map, pos: Pos) -> Set {
    let (x, y) = pos;
    (x - 1..=x + 1).cartesian_product(y - 1..=y + 1)
        .filter(|(x_, y_)| *x_ != x || *y_ != y)
        .filter(|pos| map.contains_key(&pos))
        .collect()
}

fn get_open(map: &Map, closed: &Set) -> Set {
    map.iter()
        .filter(|(_, &v)| v > 9 )
        .map(|(k, _)| k)
        .filter(|pos| !closed.contains(pos))
        .cloned()
        .collect()
}

#[allow(dead_code)]
fn display(map: &Map, dim: i128) {
    for y in 0..dim {
        for x in 0..dim {
            print!("{}", map[&(x, y)]);
        }
        println!();
    }
    println!();
} 

fn star2(input: String) -> i128 {
    let (mut map, _) = parse_input(&input);

    let mut i = 1;
    loop {
        let (new_map, _) = evolve(map);
        map = new_map;

        if map.values().all(|&v| v == 0) {
            return i;
        }

        i += 1;
    }
}


