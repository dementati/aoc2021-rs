use std::fs;

pub fn read_integers(file: &str) -> Vec<i32> {
    let contents = fs::read_to_string(file)
        .expect("Couldn't read file");

    contents.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}