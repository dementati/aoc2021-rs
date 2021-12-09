use std::collections::{HashMap, HashSet};

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

type Pos = (usize, usize);
type Map = HashMap<Pos, u32>;

fn star1(input: String) -> i128 {
    let map: Map = parse_map(&input);

    map.keys()
        .filter(|&pos| 
            neighbours(&map, pos.clone()).iter()
                .all(|n| map[pos] < map[n])
        )
        .map(|pos| map[pos] as i128 + 1)
        .sum::<i128>()
}

fn parse_map(input: &str) -> Map {
    input.split("\n").enumerate()
        .flat_map(|(y, line)| 
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x, y), c.to_digit(10).unwrap()))
        )
        .collect()
}

fn neighbours(map: &Map, pos: Pos) -> HashSet<Pos> {
    let (x, y) = pos;
    let mut result: HashSet<Pos> = HashSet::new();
    if x > 0 && map.contains_key(&(x - 1, y)) {
        result.insert((x - 1, y));
    }
    if map.contains_key(&(x + 1, y)) {
        result.insert((x + 1, y));
    }
    if y > 0 && map.contains_key(&(x, y - 1)) {
        result.insert((x, y - 1));
    }
    if map.contains_key(&(x, y + 1)) {
        result.insert((x, y + 1));
    }

    result
}

fn star2(input: String) -> i128 {
    let map: Map = parse_map(&input);

    let mut basin_sizes: Vec<usize> = map.keys()
        .filter(|&pos| 
            neighbours(&map, pos.clone()).iter()
                .all(|n| map[pos] < map[n])
        )
        .map(|&low| {
            let mut open: HashSet<Pos> = HashSet::new();
            let mut closed: HashSet<Pos> = HashSet::new();
            open.insert(low);
            while open.len() > 0 {
                let mut new_opened: HashSet<Pos> = HashSet::new();
                let mut new_closed: HashSet<Pos> = HashSet::new();
                for &pos in open.iter() {
                    let n: HashSet<Pos> = neighbours(&map, pos.clone()).iter()
                        .filter(|n| !closed.contains(n) && map[n] != 9)
                        .cloned()
                        .collect();

                    new_closed.insert(pos);
                    new_opened.extend(&n);
                }
                closed.extend(&new_closed);
                open.extend(&new_opened);
                open = &open - &new_closed;
            }

            closed.len()
        })
        .collect();

    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).fold(1, |a, b| a * b) as i128
}
