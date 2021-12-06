use itertools::izip;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let lines: Vec<&str> = input.split_whitespace().collect();
    let total = lines.len() as u32;
    let counts: Vec<u32> = lines.iter()
        .map(|line| 
            line.chars()
                .map(|c| c.to_digit(2).unwrap())
                .collect()
        )
        .reduce(|v1: Vec<u32>, v2| 
            izip!(v1, v2)
                .map(|(a, b)| a + b)
                .collect()
        )
        .unwrap();

    let gamma = counts.iter()
        .map(|&c| if c > total / 2 { '1' } else { '0' })
        .collect::<String>();
    let gamma = i128::from_str_radix(&gamma, 2).unwrap();

    gamma * (gamma ^ 4095)
}

fn star2(input: String) -> i128 {
    let bit_strs: Vec<Vec<u32>> = input
        .split_whitespace()
        .map(|line| 
            line.chars()
                .map(|c| c.to_digit(2).unwrap())
                .collect()
        )
        .collect();

    let ogr = find_rating(bit_strs.clone(), true, 0);
    let csr = find_rating(bit_strs.clone(), false, 0);
    
    ogr * csr
}

fn find_rating(bit_strs: Vec<Vec<u32>>, most_common: bool, i: usize) -> i128 {
    let count: u32 = bit_strs.iter()
        .map(|v| v[i])
        .sum();

    let most_common_bit = 
        if count == bit_strs.len() as u32 - count {
            true
        } else {
            count > bit_strs.len() as u32 / 2
        } as u32;

    let remaining: Vec<Vec<u32>> = bit_strs.into_iter()
        .filter(|v| (most_common && v[i] == most_common_bit) || (!most_common && v[i] != most_common_bit))
        .collect();

    match remaining.len() {
        1 => to_decimal(&remaining[0]),
        _ => find_rating(remaining, most_common, i + 1),
    }  
}

fn to_decimal(bit_str: &Vec<u32>) -> i128 {
    let s: String = bit_str.iter()
        .map(|&i| char::from_digit(i, 2).unwrap())
        .collect();
    i128::from_str_radix(&s, 2).unwrap()
}
