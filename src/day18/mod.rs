use itertools::Itertools;
use math::round;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    magnitude(&sum(&input))
}

fn sum(input: &str) -> String {
    input.split_whitespace()
        .map(|s| s.to_string())
        .reduce(|a, b| add(&a, &b))
        .unwrap()
}

fn add(a: &str, b: &str) -> String {
    reduce(&format!("[{},{}]", a, b))
}

fn reduce(input: &str) -> String {
    let mut result = input.to_string();
    loop {
        if let Some(new_string) = explode(&result) {
            result = new_string;
        } else if let Some(new_string) = split(&result) {
            result = new_string;
        } else {
            break;
        }
    }
    result
}

fn explode(input: &str) -> Option<String> {
    let maybe_bounds = find_explodable_pair(input);
    if maybe_bounds.is_none() {
        return None;
    }
    let (start, end) = maybe_bounds.unwrap();
    let (a, b) = parse_pair(&input[start..=end]);

    let mut new_string = input.to_string();
    new_string.replace_range(start..=end, "0");
    let mut index = start;

    if let Some((start, end)) = find_number_left(&new_string, index) {
        let num: i32 = new_string[start..=end].parse().unwrap();
        let num = num + a;
        let num = num.to_string();
        new_string.replace_range(start..=end, &num);
        let delta = num.len() - (end - start + 1);
        index += delta;
    }

    if let Some((start, end)) = find_number_right(&new_string, index) {
        let num: i32 = new_string[start..=end].parse().unwrap();
        let num = num + b;
        let num = num.to_string();
        new_string.replace_range(start..=end, &num);
    }

    Some(new_string)
}

fn find_explodable_pair(input: &str) -> Option<(usize, usize)> {
    let mut count = 0;
    let mut start: i32 = -1;
    for (i, c) in input.chars().enumerate() {
        if c == '[' {
            count += 1;

            if count == 5 {
                start = i as i32;
            }
        } 

        if c == ']' {
            count -= 1;

            if start != -1 {
                return Some((start as usize, i));
            }
        }
    }

    None
}

fn parse_pair(input: &str) -> (i32, i32) {
    input[1..(input.len() - 1)].split(",")
        .tuples::<(&str, &str)>()
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .next()
        .unwrap()
}

fn find_number_left(input: &str, index: usize) -> Option<(usize, usize)> {
    let mut end = -1;
    for (i, c) in input[0..index].chars().enumerate().collect::<Vec<_>>().iter().rev() {
        if end == -1 && c.is_digit(10) {
            end = *i as i32;
        }

        if end != -1 && !c.is_digit(10) {
            return Some((i + 1, end as usize));
        }
    }

    None
}

fn find_number_right(input: &str, index: usize) -> Option<(usize, usize)> {
    let mut start = -1;
    for (i, c) in input[(index + 1)..].chars().enumerate() {
        let i = i + index + 1;
        if start == -1 && c.is_digit(10) {
            start = i as i32;
        }

        if start != -1 && !c.is_digit(10) {
            return Some((start as usize, i - 1));
        }
    }

    None
}

fn split(input: &str) -> Option<String> {
    find_splittable_number(input)
        .map(|(start, end)| {
            let num: i32 = input[start..=end].parse().unwrap();
            let a = round::half_down(num as f64 / 2.0, 0) as i32;
            let b = round::half_up(num as f64 / 2.0, 0) as i32;
            let pair = format!("[{},{}]", a, b);
            let mut new_string = input.to_string();
            new_string.replace_range(start..=end, &pair);
            new_string
        })
}

fn find_splittable_number(input: &str) -> Option<(usize, usize)> {
    let mut start = -1;
    for (i, c) in input.chars().enumerate() {
        if start == -1 && c.is_digit(10) {
            start = i as i32;
        }

        if start != -1 && !c.is_digit(10) {
            if start as usize == i - 1 {
                start = -1;
                continue;
            }

            return Some((start as usize, i - 1));
        }
    }

    None
}

fn magnitude(input: &str) -> i128 {
    let chars: Vec<_> = input.chars().collect();
    
    let index;
    let a = if chars[1].is_digit(10) {
        index = 3;
        chars[1].to_digit(10).unwrap() as i128
    } else {
        let bi = find_matching_bracket(input, 1);
        index = bi + 2;
        magnitude(&input[1..=bi])
    };

    let b = if chars[index].is_digit(10) {
        chars[index].to_digit(10).unwrap() as i128
    } else {
        let bi = find_matching_bracket(input, index);
        magnitude(&input[index..=bi])
    };

    3*a + 2*b
}

fn find_matching_bracket(input: &str, index: usize) -> usize {
    let mut count = 0;
    for (i, c) in input.chars().enumerate() {
        if i < index {
            continue;
        }

        match c {
            '[' => count += 1,
            ']' => count -= 1,
            _ => (),
        }

        if count == 0 {
            return i;
        }
    }

    panic!("input string {} unbalanced", input);
}

fn star2(input: String) -> i128 {
    let numbers: Vec<_> = input.split_whitespace().collect();
    numbers.iter().cartesian_product(numbers.iter())
        .map(|(x, y)| magnitude(&add(x, y)))
        .max()
        .unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explode() {
        assert_eq!(explode("").is_some(), false);
        assert_eq!(explode("10").is_some(), false);
        assert_eq!(explode("[10,20]").is_some(), false);

        assert_exploded("[[[[[10,20]]]]]", "[[[[0]]]]");
        assert_exploded("[[[[[9,8],12],2],3],4]", "[[[[0,20],2],3],4]");
        assert_exploded("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        assert_exploded("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
    }

    #[test]
    fn test_explode_specific() {
        assert_exploded("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
    }

    #[test]
    fn test_split() {
        assert_split("[10,0]", "[[5,5],0]");
        assert_split("[11,0]", "[[5,6],0]");
        assert_split("[12,0]", "[[6,6],0]");
    }

    #[test]
    fn test_split_specific() {
        assert_split("[10,0]", "[[5,5],0]");
    }

    #[test]
    fn test_reduce() {
        assert_eq!(
            add("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"), 
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        )
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum("[1,1] [2,2] [3,3] [4,4]"), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(
            sum(
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
                [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
                [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
                [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
                [7,[5,[[3,8],[1,4]]]]
                [[2,[2,2]],[8,[8,1]]]
                [2,9]
                [1,[[[9,3],9],[[9,0],[0,7]]]]
                [[[5,[7,4]],7],1]
                [[[[4,2],2],6],[8,7]]"
            ), 
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(magnitude("[[1,2],[[3,4],5]]"), 143);
        assert_eq!(magnitude("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), 1384);
    }

    fn assert_exploded(input: &str, expected: &str) {
        let exploded = explode(input);
        assert_eq!(exploded.is_some(), true);
        let exploded = exploded.unwrap();
        assert_eq!(exploded, expected);
    }

    fn assert_split(input: &str, expected: &str) {
        let splitted = split(input);
        assert_eq!(splitted.is_some(), true);
        let splitted = splitted.unwrap();
        assert_eq!(splitted, expected);
    }
}