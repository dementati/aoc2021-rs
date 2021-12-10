use std::collections::HashMap;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    input.split_whitespace()
        .map(|line| score_error(line))
        .sum()
}

fn score_error(line: &str) -> i128 {
    let match_map: HashMap<char, char> = 
        [(')', '('), ('}', '{'), (']', '['), ('>', '<')].into_iter().collect();
    let score_map: HashMap<char, i128> = 
        [(')', 3), ('}', 1197), (']', 57), ('>', 25137)].into_iter().collect();

    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            c if ['(', '[', '{', '<'].contains(&c) => stack.push(c),
            c if [')', ']', '}', '>'].contains(&c) => {
                if match_map[&c] != stack.pop().unwrap() {
                    println!("{}", line);
                    return score_map[&c];
                }
            },
            _ => unreachable!(),
        }
    }

    0
}

fn star2(input: String) -> i128 {
    let mut scores: Vec<i128> = input.split_whitespace()
        .map(|line| score_complete(line))
        .filter(|&s| s != 0)
        .collect();

    scores.sort();

    println!("{:?}", scores);

    scores[scores.len() / 2]
}

fn score_complete(line: &str) -> i128 {
    let match_map: HashMap<char, char> = 
        [(')', '('), ('}', '{'), (']', '['), ('>', '<')].into_iter().collect();
    let score_map: HashMap<char, i128> = 
        [('(', 1), ('[', 2), ('{', 3), ('<', 4)].into_iter().collect();

    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            c if ['(', '[', '{', '<'].contains(&c) => stack.push(c),
            c if [')', ']', '}', '>'].contains(&c) => {
                if match_map[&c] != stack.pop().unwrap() {
                    println!("{}", line);
                    return 0;
                }
            },
            _ => unreachable!(),
        }
    }

    if stack.len() == 0 {
        return 0;
    }

    let mut total = 0;
    while !stack.is_empty() {
        total *= 5;
        total += score_map[&stack.pop().unwrap()];
    }

    total
}


