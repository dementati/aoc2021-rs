use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

type Pos = (i128, i128);
type Risk = u32;

struct Grid {
    data: Vec<Risk>,
    dim: usize,
}

impl Grid {
    fn new(data: Vec<Risk>, dim: usize) -> Grid {
        Grid { data: data, dim: dim }
    }

    fn get(&self, pos: Pos) -> Option<Risk> {
        let (x, y) = pos;
        
        if x < 0 || y < 0 || x >= self.dim() || y >= self.dim() {
            return None;
        }

        let i = y * self.dim as i128 + x;
        if i >= 0 && i < self.data.len() as i128 {
            Some(self.data[i as usize])
        } else {
            None
        }
    }

    fn neighbours(&self, pos: Pos) -> HashSet<Pos> {
        let (x, y) = pos;
        [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter()
            .filter(|pos| self.get(*pos).is_some())
            .collect()
    }

    fn dim(&self) -> i128 {
        self.dim as i128
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: Risk,
    position: Pos,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn star1(input: String) -> i128 {
    let grid = parse_input(&input);
    shortest(&grid, (0, 0), (grid.dim() - 1, grid.dim() - 1))
}

fn parse_input(input: &str) -> Grid {
    let dim = input.split_whitespace().count();
    let data: Vec<Risk> = input.split_whitespace()
        .flat_map(|line| line.chars())
        .map(|c| c.to_digit(10).unwrap() as Risk)
        .collect();
    Grid::new(data, dim)
}

fn shortest(grid: &Grid, start: Pos, goal: Pos) -> i128 {
    let h = |(x, y): Pos| -> Risk {
        let (gx, gy) = goal;
        ((gx - x).pow(2) as f64 + (gy - y).pow(2) as f64).sqrt() as Risk
    };

    let mut open = BinaryHeap::new();
    open.push(State { cost: h(start), position: start });
    let mut closed = HashSet::new();

    let mut g = HashMap::new();
    g.insert(start, 0);

    let mut came_from = HashMap::new();

    while !open.is_empty() {
        let current = { 
            let mut candidate = open.pop().unwrap().position;
            while closed.contains(&candidate) {
                candidate = open.pop().unwrap().position;
            }
            candidate
        };
        
        closed.insert(current);
        
        if current == goal {
            return score(grid, came_from, current) as i128;
        }

        for neighbour in grid.neighbours(current) {
            let tentative_g = g[&current] + grid.get(neighbour).unwrap();
            if tentative_g < *g.entry(neighbour).or_insert(Risk::MAX) {
                came_from.insert(neighbour, current);
                g.insert(neighbour, tentative_g);
                open.push( State { cost: tentative_g + h(neighbour), position: neighbour } )
            }
        }
    }

    panic!("No path found");
}

fn score(grid: &Grid, came_from: HashMap<Pos, Pos>, goal: Pos) -> u32 {
    let mut score = 0;
    let mut current = goal;
    let mut path = HashSet::new();
    while came_from.contains_key(&current) {
        path.insert(current);
        score += grid.get(current).unwrap();
        current = came_from[&current];
    }

    score
}

fn star2(input: String) -> i128 {
    let grid = parse_input2(&input);
    shortest(&grid, (0, 0), (grid.dim() - 1, grid.dim() - 1))
}

fn parse_input2(input: &str) -> Grid {
    let dim = input.split_whitespace().count();
    let data: Vec<Risk> = input.split_whitespace()
        .flat_map(|line| line.chars())
        .map(|c| c.to_digit(10).unwrap() as Risk)
        .collect();

    let gdim = dim * 5;

    let mut new_data = vec![0; gdim.pow(2)];
    for sy in 0..5 {
        for sx in 0..5 {
            let offset = (sx + sy) as u32;
            for y in 0..dim {
                for x in 0..dim {
                    let gx = sx * dim + x;
                    let gy = sy * dim + y;
                    let v = data[y * dim + x];
                    new_data[gy * gdim + gx] = (v - 1 + offset) % 9 + 1;
                }
            }
        }
    }

    Grid::new(new_data, dim * 5)
}
