use regex::Regex;

pub fn read_integers(input: &str) -> Vec<i32> {
    input.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn read_labeled_integers(input: &str) -> Vec<(&str, i32)> {
    let re = Regex::new(r"(\w+)\s+([-\d]+)").unwrap();

    input.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| re.captures(line).unwrap())
        .map(|caps| (
            caps.get(1).unwrap().as_str(), 
            caps.get(2).unwrap().as_str().parse().unwrap(),
        ))
        .collect()
}
