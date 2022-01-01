use std::collections::HashSet;

type Pos = (usize, usize);
type Herd = HashSet<Pos>;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let (mut east, mut south, w, h) = parse_input(&input);

    let mut step = 0;
    loop {
        step += 1;
        let (new_east, new_south, changed) = evolve(&east, &south, w, h);

        if !changed {
            break;
        }

        east = new_east;
        south = new_south;
    }

    step as i128
}

fn parse_input(input: &str) -> (Herd, Herd, usize, usize) {
    let mut east = Herd::new();
    let mut south = Herd::new();

    for (y, line) in input.split_whitespace().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '>' => { east.insert((x, y)); },
                'v' => { south.insert((x, y)); },
                _ => (),
            }
        }
    }

    let h = input.split_whitespace().count();
    let w = input.split_whitespace().next().unwrap().chars().count();

    (east, south, w, h)
}

fn evolve(east: &Herd, south: &Herd, w: usize, h: usize) -> (Herd, Herd, bool) {
    let mut new_east = Herd::new();
    let mut new_south = Herd::new();
    let mut changed = false;

    for pos in east.iter() {
        let (x, y) = *pos;
        let candidate = ((x + 1) % w, y);

        if east.contains(&candidate) || south.contains(&candidate) {
            new_east.insert(*pos);
        } else {
            new_east.insert(candidate);
            changed = true;
        }
    }

    for pos in south.iter() {
        let (x, y) = *pos;
        let candidate = (x, (y + 1) % h);

        if new_east.contains(&candidate) || south.contains(&candidate) {
            new_south.insert(*pos);
        } else {
            new_south.insert(candidate);
            changed = true;
        }
    }

    (new_east, new_south, changed)
}

fn display(east: &Herd, south: &Herd, w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            let pos = (x, y);
            if east.contains(&pos) {
                print!(">");
            } else if south.contains(&pos) {
                print!("v");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn star2(input: String) -> i128 {
    0
}
