pub fn solver(star: u8) -> fn(String) -> i32 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i32 {
    let mut data: Vec<i32> = input.split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    evolve(&mut data, 1, 80)
}

fn evolve(data: &mut Vec<i32>, gen: i32, max_gen: i32) -> i32 {
    let mut new = 0;
    for v in data.iter_mut() {
        *v -= 1;
        if v == &-1 {
            *v = 6;
            new += 1;
        }
    }

    for _ in 0..new {
        data.push(8);
    }

    return if gen == max_gen {
        data.len() as i32
    } else {
        evolve(data, gen + 1, max_gen)
    };
}

fn star2(input: String) -> i32 {
    0
}
