use itertools::Itertools;

pub fn read_integers(input: &str) -> Option<Vec<i32>> {
    input.split_whitespace()
        .map(|line| line.parse().ok())
        .collect()
}

pub fn read_labeled_integers(input: &str) -> Option<Vec<(&str, i32)>> {
    input.split_whitespace()
        .tuples::<(&str, &str)>()
        .map(|(a, b)| match b.parse() {
            Ok(b) => Some((a, b)),
            _ => None,
        })
        .collect()
}
