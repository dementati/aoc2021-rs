use std::cmp::max;

use regex::Regex;

type Pos = (i128, i128);
type Target = (Pos, Pos);

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let target = parse_input(&input);
    let mut max_y = 0;
    for vy in 0..1000 {
        for vx in 0..1000 {
            if let Some(y) = simulate((vx, vy), target) {
                max_y = max(max_y, y);
            }
        }
    }

    max_y
}

fn parse_input(input: &str) -> Target {
    let re = Regex::new(r"^target area: x=([-\d]+)..([-\d]+), y=([-\d]+)..([-\d]+)$").unwrap();
    let caps = re.captures(input).unwrap();
    (
        (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
        (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
    )
}

fn inside(pos: Pos, target: Target) -> bool {
    let (x, y) = pos;
    let (mx, my) = target;
    let (min_x, max_x) = mx;
    let (min_y, max_y) = my;

    x >= min_x && x <= max_x && y >= min_y && y <= max_y
}

fn simulate(v0: Pos, target: Target) -> Option<i128> {
    let (_, (y_limit, _)) = target;
    let mut pos = (0, 0);
    let mut max_y = 0;
    let mut v = v0;
    loop {
        let (mut x, mut y) = pos;
        let (vx, vy) = v;
        x += vx;
        y += vy;
        v = (max(vx - 1, 0), vy - 1);
        max_y = max(y, max_y);
        pos = (x, y);

        if inside(pos, target) {
            return Some(max_y);
        } else if y < y_limit {
            return None;
        }
    }
}

fn star2(input: String) -> i128 {
    let target = parse_input(&input);
    let mut count = 0;
    for vy in -1000..1000 {
        for vx in 0..1000 {
            if simulate((vx, vy), target).is_some() {
                count += 1;
            }
        }
    }

    count
}
