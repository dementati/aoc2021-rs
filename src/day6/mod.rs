
pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let data = parse_input(&input);
    evolve(&data, 1, 80)
}

fn star2(input: String) -> i128 {
    let data = parse_input(&input);
    evolve(&data, 1, 256)
}

fn parse_input(input: &str) -> Vec<i128> {
    let input: Vec<i128> = input.split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut data = vec![0; 9];

    for &v in input.iter() {
        data[v as usize] += 1;
    }

    data
}

fn evolve(data: &Vec<i128>, gen: i128, max_gen: i128) -> i128 {
    let mut new_data = vec![0; 9];
    for i in 1..=8 {
        new_data[i-1] = data[i];
    }
    new_data[8] = data[0];
    new_data[6] += data[0];

    match gen {
        _ if (gen == max_gen) => new_data.iter().sum::<i128>(),
        _ => evolve(&new_data, gen + 1, max_gen),
    }
}
