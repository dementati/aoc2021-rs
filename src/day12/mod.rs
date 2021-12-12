use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

type Graph = HashMap<String, HashSet<String>>;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let graph = parse_input(&input);
    search(&graph, "start", hashset!{"start"}, false) as i128
}

fn parse_input(input: &str) -> Graph {
    let tuples: Vec<(&str, &str)> = input.split_whitespace()
        .flat_map(|line| line.split('-'))
        .tuples::<(&str, &str)>()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    for (_, g) in &tuples.into_iter().group_by(|(k, _)| *k) {
        for (k, v) in g {
            map.entry(k.to_string()).or_default().insert(v.to_string());
            map.entry(v.to_string()).or_default().insert(k.to_string());
        }
    }

    map
}

fn search(
    graph: &Graph, 
    node: &str, 
    closed: HashSet<&str>,
    bonus: bool,
) -> i128 {
    let mut count = 0;
    for other in graph.get(node).unwrap() {
        if other.eq("end") {
            count += 1;
            continue;
        }

        if other.eq("start") {
            continue;
        }

        if other.chars().all(char::is_lowercase) {
            if !closed.contains(other as &str) {
                let mut closed = closed.clone();
                closed.insert(other);
                count += search(graph, other, closed, bonus);
            } else if bonus {
                count += search(graph, other, closed.clone(), false);
            }
        } else {
            count += search(graph, other, closed.clone(), bonus);
        }
    }

    count
}

fn star2(input: String) -> i128 {
    let graph = parse_input(&input);
    search(&graph, "start", hashset!{"start"}, true) as i128
}
