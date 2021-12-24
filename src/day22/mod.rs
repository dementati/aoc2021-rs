use std::collections::HashSet;
use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

type Coord = i32;
type Pos = (Coord, Coord, Coord);
type Cuboid = (Pos, Pos);

fn star1(input: String) -> i128 {
    let instructions = parse_input(&input);

    let limit = ((-50, -50, -50), (50, 50, 50));

    let mut map = HashSet::new();
    for (set, cuboid) in instructions.iter() {
        if !overlaps(&limit, cuboid) {
            continue;
        }

        evolve(&mut map, *set, *cuboid, limit);
    }

    map.len() as i128
}

fn parse_input(input: &str) -> Vec<(bool, Cuboid)> {
    let re = Regex::new(r"^x=([-\d]+)..([-\d]+),y=([-\d]+)..([-\d]+),z=([-\d]+)..([-\d]+)$").unwrap();

    input.split_whitespace()
        .tuples::<(&str, &str)>()
        .map(|(on, cuboid)|
            (
                on == "on",
                {
                    let caps = re.captures(cuboid).unwrap();
                    (
                        (
                            caps[1].parse().unwrap(),
                            caps[3].parse().unwrap(),
                            caps[5].parse().unwrap(),
                        ),
                        (
                            caps[2].parse().unwrap(),
                            caps[4].parse().unwrap(),
                            caps[6].parse().unwrap(),
                        ),
                    )
                }
            )
        )
        .collect()
}

fn evolve(map: &mut HashSet<Pos>, set: bool, cuboid: Cuboid, limit: Cuboid) {
    let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = cuboid;
    for z in min_z..=max_z {
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pos = (x, y, z);

                if !contains(limit, pos) {
                    continue;
                }

                if set {
                    map.insert(pos);
                } else {
                    map.remove(&pos);
                }
            }
        }
    } 
}

fn contains(cuboid: Cuboid, pos: Pos) -> bool {
    let (x, y, z) = pos;
    let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = cuboid;

    (min_x..=max_x).contains(&x) && 
    (min_y..=max_y).contains(&y) &&
    (min_z..=max_z).contains(&z)
}

fn overlaps(a: &Cuboid, b: &Cuboid) -> bool {
    let ((min_x1, min_y1, min_z1), (max_x1, max_y1, max_z1)) = a;
    let ((min_x2, min_y2, min_z2), (max_x2, max_y2, max_z2)) = b;

    min_x1 <= max_x2 && max_x1 >= min_x2 &&
    min_y1 <= max_y2 && max_y1 >= min_y2 &&
    min_z1 <= max_z2 && max_z1 >= min_z2
}

fn star2(input: String) -> i128 {
    let instructions = parse_input(&input);
    let overlapping_cuboids = instructions.iter().map(|(_, cuboid)| *cuboid).collect();
    let nonoverlapping_cuboids = subdivide(&overlapping_cuboids);

    println!("Subdivision complete");

    let mut lit_set = hashset!{};
    for other in nonoverlapping_cuboids.into_iter() {
        for (lit, cuboid) in instructions.iter() {
            if overlaps(cuboid, &other) {
                if *lit {
                    lit_set.insert(other);
                } else {
                    lit_set.remove(&other);
                }
            }
        }
    }

    lit_set.iter().map(|c| len(*c)).sum::<i128>()
}

fn len(cuboid: Cuboid) -> i128 {
    let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = cuboid;

    (max_x as i128 + 1 - min_x as i128) * (max_y as i128 + 1 - min_y as i128) * (max_z as i128 + 1 - min_z as i128)
}

#[derive(Debug, Clone)]
struct Interval {
    start: i32,
    end: i32,
    ids: HashSet<usize>,
}

#[derive(Debug)]
struct Checkpoint {
    value: i32,
    start_ids: HashSet<usize>,
    end_ids: HashSet<usize>,
}

fn subdivide(cuboids: &Vec<Cuboid>) -> Vec<Cuboid> {
    // Find labeled intervals
    let (xcp, ycp, zcp) = checkpoints(cuboids);

    println!("Checkpoints computed: ({}, {}, {})", xcp.len(), ycp.len(), zcp.len());

    let xins = intervals(&xcp);
    let yins = intervals(&ycp);
    let zins = intervals(&zcp);

    println!("Intervals computed: ({}, {}, {})", xins.len(), yins.len(), zins.len());

    let mut result = Vec::new();
    // For all triples of intervals
    for (i, zin) in zins.iter().enumerate() {
        println!("Finished zin {} out of {}", i + 1, zins.len());
        for yin in yins.iter() {
            let zin_u_yin = &zin.ids & &yin.ids;
            for xin in xins.iter() {
                // Create a box if index sets overlap
                if !(&zin_u_yin & &xin.ids).is_empty() {
                    result.push(from_intervals(xin, yin, zin));
                }
            }
        }
        println!("result size: {}", result.len());
    }

    result
}

fn checkpoints(cuboids: &Vec<Cuboid>) -> (Vec<Checkpoint>, Vec<Checkpoint>, Vec<Checkpoint>) {
    let mut xcps = axis_checkpoints(cuboids, |((x, _, _), _)| x, |(_, (x, _, _))| x);
    xcps.sort_by_key(|cp| cp.value);

    let mut ycps = axis_checkpoints(cuboids, |((_, x, _), _)| x, |(_, (_, x, _))| x);
    ycps.sort_by_key(|cp| cp.value);

    let mut zcps = axis_checkpoints(cuboids, |((_, _, x), _)| x, |(_, (_, _, x))| x);
    zcps.sort_by_key(|cp| cp.value);

    (xcps, ycps, zcps)
}

fn axis_checkpoints(
    cuboids: &Vec<Cuboid>, 
    start_axis: fn (Cuboid) -> Coord,
    end_axis: fn (Cuboid) -> Coord,
) -> Vec<Checkpoint> {
    let mut axes: Vec<_> = cuboids.iter().enumerate()
        .map(|(i, cuboid)| (start_axis(*cuboid), i, true))
        .collect();
    let end_axes: Vec<_> = cuboids.iter().enumerate()
        .map(|(i, cuboid)| (end_axis(*cuboid) + 1, i, false))
        .collect();

    axes.extend(end_axes);
    axes.sort_by_key(|(x, _, _)| *x);

    axes.into_iter()
        .group_by(|(x, _, _)| *x)
        .into_iter()
        .map(|(x, g)| {
                let mut g: HashMap<_, _> = g.map(|(_, i, start)| (i, start))
                    .group_by(|(_, start)| *start)
                    .into_iter()
                    .map(|(start, g)| (start, g.into_iter().map(|(i, _)| i).collect::<HashSet<_>>()))
                    .collect();

                Checkpoint {
                    value: x,
                    start_ids: g.entry(true).or_default().clone(),
                    end_ids: g.entry(false).or_default().clone(),
                }
            }
        )
        .collect()
}

fn intervals(checkpoints: &Vec<Checkpoint>) -> Vec<Interval> {
    let mut ids = HashSet::new();
    let mut result = Vec::new();
    let mut it = checkpoints.iter().tuple_windows().peekable();
    while let Some((cp, cp_next)) = it.next() {
        ids.extend(cp.start_ids.iter());
        for id in cp.end_ids.iter() {
            ids.remove(id);
        }

        let interval = Interval { start: cp.value, end: cp_next.value, ids: ids.clone() };
        result.push(interval);

        if it.peek().is_none() {
            result.push(Interval { start: cp.value, end: cp_next.value, ids: ids.clone() });
        }
    }

    result
}

fn from_intervals(xin: &Interval, yin: &Interval, zin: &Interval) -> Cuboid {
    (
        (
            xin.start,
            yin.start,
            zin.start,
        ),
        (
            xin.end - 1,
            yin.end - 1,
            zin.end - 1,
        )
    )
}
