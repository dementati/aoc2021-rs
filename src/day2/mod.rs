use crate::common;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let (x, y) = common::read_labeled_integers(&input).unwrap()
        .into_iter()
        .map(|(label, value)| match label {
            "forward" => (value, 0),
            "up" => (0, -value),
            "down" => (0, value),
            _ => panic!("Unknown label"),
        })
        .fold((0, 0), |(sx, sy), (x, y)| (sx + x, sy + y));

    x * y
}

fn star2(input: String) -> i128 {
    let mut aim = 0;
    let mut cur_pos = (0, 0);
    for (label, value) in common::read_labeled_integers(&input).unwrap() {
        match label {
            "forward" => { 
                let (x, y) = cur_pos;
                cur_pos = (x + value, y + aim * value);
            },
            "up" => aim -= value,
            "down" => aim += value,
            _ => panic!("Unexpected label {}", label),
        }
    }

    let (x, y) = cur_pos;
    x * y
}
