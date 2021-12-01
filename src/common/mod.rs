pub fn read_integers(input: &str) -> Vec<i32> {
    input.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}
