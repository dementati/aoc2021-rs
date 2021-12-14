use std::collections::HashMap;

use itertools::Itertools;

type State = HashMap<(char, char), i128>;
type Rules = HashMap<(char, char), char>;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    run(&input, 10)
}

fn parse_input(input: &str) -> (State, Rules, char) {
    let (template, rules) = input.split("\n\n")
        .tuples::<(&str, &str)>()
        .next()
        .unwrap();

    let rules = rules.split_whitespace()
        .tuples::<(&str, &str, &str)>()
        .map(|(k, _, v)| (
            k.chars().tuples::<(char, char)>().next().unwrap(), 
            v.chars().next().unwrap())
        )
        .collect();

    let mut state = State::new();
    for pair in template.chars().tuple_windows() {
        *state.entry(pair).or_default() += 1;
    }

    (state, rules, template.chars().next().unwrap())
}

fn pass(state: &State, rules: &Rules) -> State {
    let mut new_state = State::new();
    for ((a, c), count) in state.iter() {
        let b = rules[&(*a, *c)];
        *new_state.entry((*a, b)).or_default() += count;
        *new_state.entry((b, *c)).or_default() += count;
    }

    new_state
}

fn score(state: &State, first: char) -> i128 {
    let mut count: HashMap<char, i128> = HashMap::new();
    *count.entry(first).or_default() += 1;
    for ((_, c), n) in state.iter() {
        *count.entry(*c).or_default() += n;
    }

    let mx = count.values().max().unwrap();
    let mn = count.values().min().unwrap();

    mx - mn
}

fn run(input: &str, passes: u32) -> i128 {
    let (state, rules, first) = parse_input(&input);

    let mut state = state;
    for _ in 0..passes {
        state = pass(&state, &rules);
    }

    score(&state, first)
}

fn star2(input: String) -> i128 {
    run(&input, 40)
}
