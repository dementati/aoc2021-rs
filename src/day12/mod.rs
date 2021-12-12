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
    let result = search(&graph, "start", HashSet::new(), Vec::new());
    result.len() as i128
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

fn search<'a>(
    graph: &'a Graph, 
    node: &'a str, 
    visited: HashSet<&str>, 
    path: Vec<&'a str>
) -> Vec<Vec<&'a str>> {
    let to_visit: HashSet<_> = graph.get(node).unwrap_or_else(|| panic!("{} not found", node)).iter()
                .filter(|&other| 
                    !(other.chars().all(char::is_lowercase) && visited.contains(other as &str))
                )
                .collect();

    let mut result = Vec::new();
    for other in to_visit.iter() {
        let mut new_visited = visited.clone();
        new_visited.insert(node);
        let mut new_path = path.clone();
        new_path.push(node);

        if other.eq(&"end") {
            new_path.push(other);
            result.push(new_path);
        } else {
            let new_result = search(graph, other, new_visited, new_path);
            result.extend(new_result);
        }
    }

    result
}

fn star2(input: String) -> i128 {
    let graph = parse_input(&input);
    let mut total_result = HashSet::new();
    for double in get_small_caves(&graph) {
        let result = search2(&graph, "start", HashSet::new(), Vec::new(), &double);
        total_result.extend(result);
    }

    total_result.len() as i128
}

fn search2<'a>(
    graph: &'a Graph, 
    node: &'a str, 
    visited: HashSet<&str>, 
    path: Vec<&'a str>,
    double: &str,
) -> Vec<Vec<&'a str>> {
    let to_visit: HashSet<_> = graph.get(node).unwrap_or_else(|| panic!("{} not found", node)).iter()
                .filter(|&other| 
                    !(other.chars().all(char::is_lowercase) && visited.contains(other as &str))
                    ||
                    (
                        other.chars().all(char::is_lowercase) &&
                        other.eq(double) &&
                        path.iter()
                            .filter(|&n| n.eq(other))
                            .count() == 1
                    )
                )
                .collect();

    let mut result = Vec::new();
    for other in to_visit.iter() {
        let mut new_visited = visited.clone();
        new_visited.insert(node);
        let mut new_path = path.clone();
        new_path.push(node);

        if other.eq(&"end") {
            new_path.push(other);
            result.push(new_path);
        } else {
            let new_result = search2(graph, other, new_visited, new_path, double);
            result.extend(new_result);
        }
    }

    result
}

fn get_small_caves(graph: &Graph) -> Vec<String> {
    graph.keys()
        .filter(|k| !["start", "end"].contains(&k.as_str()) && k.chars().all(char::is_lowercase))
        .cloned()
        .collect()
}
