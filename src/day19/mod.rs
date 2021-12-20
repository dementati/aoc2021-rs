use std::collections::{HashMap, HashSet};

use itertools::Itertools;
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

fn rotate(pos: Pos, rotation: Rotation) -> Pos {
    let point = pos.cast::<f64>();
    let rotated = rotation * point;
    Pos::new(
        rotated.x.round() as i128,
        rotated.y.round() as i128,
        rotated.z.round() as i128,
    )
}

#[derive(Eq, PartialEq, Hash, Clone)]
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
        for i in 0..self.len() {
            let norm_a = self.normalize(i);
            for j in 0..other.len() {
                for rotation in rotations() {
                    let rotated_b = other.rotate(rotation);
                    let norm_b = rotated_b.normalize(j);

                    if norm_a.intersection(&norm_b).count() >= 12 {
                        let pos = self.get(i) - rotated_b.get(j);
                        return Some((pos, rotation));
                    }
                }
            }
        }

        None
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
    let locations = locate_scanners(scanners[0].clone(), scanners);

    let beacons: HashSet<Pos> = locations.iter()
        .flat_map(|(s, pos)| 
            s.beacons.iter()
                .map(move |b| pos + b)
        )
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

/// Find locations and rotations of scanners relative to origin.
/// Result map scanner keys are rotated to align with origin.
fn locate_scanners<'a>(origin: Scanner, scanners: Vec<Scanner>) -> HashMap<Scanner, Pos> {
    let mut open = hashset!{origin.clone()};
    let mut closed: HashSet<_> = scanners.into_iter().filter(|s| s != &origin).collect();

    let mut result: HashMap<Scanner, Pos> = hashmap!{origin => Pos::new(0, 0, 0)};
    while !open.is_empty() {
        let s = open.iter().next().unwrap().clone();
        open.remove(&s);

        let mut new_closed = hashset!{};
        for s2 in closed.iter() {
            if let Some((pos, rotation)) = s.locate(&s2) {
                let rotated_s2 = s2.rotate(rotation);
                let s_pos = result[&s];
                let s2_pos = s_pos + pos;
                result.insert(rotated_s2.clone(), s2_pos);
                open.insert(rotated_s2);
            } else {
                new_closed.insert(s2.clone());
            }
        }
        closed = new_closed;
    }

    result
}

fn star2(input: String) -> i128 {
    let scanners = parse_input(&input);
    let locations: Vec<_> = locate_scanners(scanners[0].clone(), scanners)
        .values()
        .map(|pos| (pos.x, pos.y, pos.z))
        .collect();
    locations.iter().cartesian_product(locations.iter())
        .map(|((ax, ay, az), (bx, by, bz))| (ax - bx).abs() + (ay - by).abs() + (az - bz).abs())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locate() {
        let input = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390";

        let scanners = parse_input(input);
        let maybe = scanners[0].locate(&scanners[1]);
        assert_eq!(maybe.is_some(), true);
        let (pos, _) = maybe.unwrap();
        assert_eq!(pos, Pos::new(68, -1246, -43));
    }
}
