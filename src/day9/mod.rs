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
    let map: Map = input.split("\n").enumerate()
        .flat_map(|(y, line)| 
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x, y), c.to_digit(10).unwrap()))
        )
        .collect();

    map.keys()
        .filter(|&pos| 
            neighbours(&map, pos.clone()).iter()
                .all(|n| map[pos] < map[n])
        )
        .map(|pos| map[pos] as i128 + 1)
        .sum::<i128>()
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
    let map: Map = input.split("\n").enumerate()
        .flat_map(|(y, line)| 
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x, y), c.to_digit(10).unwrap()))
        )
        .collect();

    let mut basin_sizes: Vec<usize> = map.keys()
        .filter(|&pos| 
            neighbours(&map, pos.clone()).iter()
                .all(|n| map[pos] < map[n])
        )
        .map(|&low| {
            let mut basin: HashSet<Pos> = HashSet::new();
            basin.insert(low);
            loop {
                let mut new_pos: HashSet<Pos> = HashSet::new();
                for pos in basin.iter() {
                    let n: HashSet<Pos> = neighbours(&map, pos.clone()).iter()
                        .filter(|n| map[n] != 9)
                        .cloned()
                        .collect();

                    new_pos = new_pos.union(&n).cloned().collect();
                }

                let prev_size = basin.len();
                basin = basin.union(&new_pos).cloned().collect();
                if basin.len() == prev_size {
                    break;
                }
            }

            basin.len()
        })
        .collect();

    basin_sizes.sort();

    basin_sizes.iter().rev().take(3).fold(1, |a, b| a * b) as i128
}
