use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use math::round;
use nalgebra::{Vector3, Rotation3};

type Pos = Vector3<i128>;
type Rotation = Rotation3<f64>;

fn rotations() -> Vec<Rotation> {
    let rad90 = std::f64::consts::FRAC_PI_2;

    let rotations = vec![
        0.0 * Vector3::y() * rad90,
        1.0 * Vector3::y() * rad90,
        2.0 * Vector3::y() * rad90,
        3.0 * Vector3::y() * rad90,
        1.0 * Vector3::x() * rad90,
        -1.0 * Vector3::x() * rad90,
    ];

    let mut result = vec!{};
    for i in 0..4 {
        let i = i as f64;
        let z_rot = Rotation3::new(i * Vector3::z() * rad90);
        for &rot in rotations.iter() {
            let xy_rot = Rotation3::new(rot);
            let rot = z_rot * xy_rot;
            result.push(rot);
        }
    }

    result
}

fn zero_rotation() -> Rotation {
    Rotation3::new(Vector3::new(0.0, 0.0, 0.0))
}

fn rotate(pos: Pos, rotation: Rotation) -> Pos {
    let point = pos.cast::<f64>();
    let rotated = rotation * point;
    Pos::new(
        round::half_up(rotated.x, 0) as i128,
        round::half_up(rotated.y, 0) as i128,
        round::half_up(rotated.z, 0) as i128,
    )
}

#[derive(Eq, PartialEq, Hash)]
struct Scanner {
    beacons: Vec<Pos>,
}

impl Scanner {
    fn new(beacons: Vec<Pos>) -> Scanner {
        Scanner { beacons: beacons }
    }

    fn len(&self) -> usize {
        self.beacons.len()
    }

    fn get(&self, index: usize) -> Pos {
        self.beacons[index]
    }

    /// Subtract the first 
    fn normalize(&self, index: usize) -> HashSet<Pos> {
        let first = self.beacons[index];
        self.beacons.iter()
            .map(|pos| pos - first)
            .collect()
    }

    fn rotate(&self, rotation: Rotation) -> Scanner {
        let beacons = self.beacons.iter()
            .map(|&pos| rotate(pos, rotation))
            .collect();

        Scanner::new(beacons)
    }

    /// Checks if the scanner has at least 12 beacons in common
    /// with another scanner, and if so, returns the relative
    /// position of other relative to this scanner
    fn locate(&self, other: &Scanner) -> Option<(Pos, Rotation)> {
        // Normalize on 0 for A
        let norm_a = self.normalize(0);

        // For each rotation of scanner B
        for rotation in rotations() {
            let rotated_b = other.rotate(rotation);
            // For each beacon x in B
            for i in 0..other.len() - 1 {
                // Normalize on x for B
                let norm_b = rotated_b.normalize(i);

                // Check if intersection is >= 12
                let intersect = norm_a.intersection(&norm_b);
                if norm_a.intersection(&norm_b).count() >= 12 {
                    // Subtract A[0] from rotated B[x] 
                    // to position of B relative to A
                    let pos = self.get(0) - rotated_b.get(i);
                    return Some((pos, rotation));
                }
            }
        }

        None
    }

    /// Get global positions of the beacons of this scanner. 
    /// global_pos is the global location of this scanner.
    /// global_rot is the global rotation of this scanner.
    fn global_beacons(&self, global_pos: Pos, global_rot: Rotation) -> HashSet<Pos> {
        self.beacons.iter()
            .map(|&pos| rotate(pos, global_rot) + global_pos)
            .collect()
    }
}

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    solve1(&input)
}

fn solve1(input: &str) -> i128 {
    let scanners = parse_input(input);
    let locations = locate_scanners(&scanners[0], &scanners);

    let beacons: HashSet<Pos> = scanners.iter()
        .flat_map(|s| {
            let (pos, rot) = locations[s];
            s.global_beacons(pos, rot)
        })
        .collect();

    beacons.len() as i128
}

fn parse_input(input: &str) -> Vec<Scanner> {
    input.split("\n\n")
        .map(|s| {
            let beacons = s.split("\n").skip(1)
                .map(|v| 
                    v.split(",")
                        .tuples::<(&str, &str, &str)>()
                        .map(|(x, y, z)| 
                            Pos::new(
                                x.parse().unwrap(),
                                y.parse().unwrap(),
                                z.parse().unwrap(),
                            )
                        )
                        .next()
                        .unwrap()
                )
                .collect();

            Scanner::new(beacons)
        })
        .collect()
}

/// Find locations and rotations of scanners relative to origin
/// Indices of scanners match indices of result.
fn locate_scanners<'a>(origin: &'a Scanner, scanners: &'a Vec<Scanner>) -> HashMap<&'a Scanner, (Pos, Rotation)> {
    let mut open = hashset!{origin};
    let mut closed: HashSet<_> = scanners.iter().filter(|&s| s != origin).collect();

    let mut result: HashMap<&Scanner, (Pos, Rotation)> = hashmap!{origin => (Pos::new(0, 0, 0), zero_rotation())};
    while !open.is_empty() {
        // Get a scanner s from S
        let s = open.iter().next().unwrap().clone();
        open.remove(&s);

        let mut new_closed = hashset!{};
        for &s2 in closed.iter() {
            if let Some((pos, rotation)) = s.locate(s2) {
                let (s_pos, _) = result[s];
                let s2_pos = s_pos + pos;
                result.insert(s2, (s2_pos, rotation));
                open.insert(s2);
            } else {
                new_closed.insert(s2);
            }
        }
        closed = new_closed;
    }

    result
}

fn star2(input: String) -> i128 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation() {
    }
}
