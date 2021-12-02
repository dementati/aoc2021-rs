use crate::common;

pub fn solver(star: u8) -> fn(String) -> i32 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i32 {
    let items: Vec<_> = common::read_labeled_integers(&input);

    let x: i32 = items.clone().into_iter()
        .filter(|(label, _)| *label == "forward")
        .map(|(_, value)| value)
        .sum();

    let y: i32 = items.into_iter()
        .map(|(label, value)| match label {
            "up" => -(value as i32),
            "down" => value as i32,
            _ => 0,
        })
        .sum();

    x * y
}

fn star2(input: String) -> i32 {
    let mut aim = 0;
    let mut cur_pos = (0, 0);
    for (label, value) in common::read_labeled_integers(&input) {
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
