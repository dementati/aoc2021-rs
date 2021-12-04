use std::collections::HashSet;

use itertools::Itertools;

pub fn solver(star: u8) -> fn(String) -> i32 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i32 {
    let input: Vec<_> = input.split_whitespace()
        .collect();

    let draws: Vec<u16> = input[0].split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let boards: &Vec<u16> = &input[1..].iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut marked = vec![false; boards.len()];

    for &draw in draws.iter() {
        for (i, &v) in boards.iter().enumerate() {
            if draw == v {
                marked[i] = true;
                if let Some(board) = check(&marked) {
                    return score(boards, &marked, board, draw);
                }
            }
        }
    }

    panic!("Couldn't find winning board")
}

fn at<T>(boards: &Vec<T>, board: u16, row: u16, col: u16) -> &T {
    let i = (board * 25 + row * 5 + col) as usize;
    &boards[i]
}

fn check(marked: &Vec<bool>) -> Option<u16> {
    for board in 0..(marked.len() / 25) {
        let board = board as u16;
        for row in 0..5 {
            if (0..5).all(|col| *at(marked, board, row, col)) {
                return Some(board);
            }
        }

        for col in 0..5 {
            if (0..5).all(|row| *at(marked, board, row, col)) {
                return Some(board);
            }
        }
    }

    None
}

fn score(
    boards: &Vec<u16>, 
    marked: &Vec<bool>, 
    board: u16, 
    winning_draw: u16
) -> i32 {
    let unmarked: Vec<u16> = (0..5).cartesian_product(0..5)
        .filter(|(row, col)| !*at(marked, board, *row, *col))
        .map(|(row, col)| *at(boards, board, row, col))
        .collect();
    let unmarked_sum: u16 = unmarked.iter().sum();
    (unmarked_sum * winning_draw) as i32
}

fn star2(input: String) -> i32 {
    let input: Vec<_> = input.split_whitespace()
        .collect();

    let draws: Vec<u16> = input[0].split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let boards: &Vec<u16> = &input[1..].iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let board_count = boards.len() / 25;

    let mut marked = vec![false; boards.len()];
    let mut winners = HashSet::new();

    for &draw in draws.iter() {
        for (i, &v) in boards.iter().enumerate() {
            if draw == v {
                marked[i] = true;
                let result = check_all(&marked, &winners);
                winners = result.0;
                let last_winner = result.1;
                if winners.len() == board_count {
                    return score(boards, &marked, last_winner, draw);
                }
            }
        }
    }

    panic!("Couldn't find winning board")
}

fn check_all(marked: &Vec<bool>, prev_winners: &HashSet<u16>) -> (HashSet<u16>, u16) {
    let mut winners = prev_winners.clone();
    let mut last_winner = 0;
    for board in 0..(marked.len() / 25) {
        let board = board as u16;
        if prev_winners.contains(&board) {
            continue;
        }

        let board = board as u16;
        for row in 0..5 {
            if (0..5).all(|col| *at(marked, board, row, col)) {
                winners.insert(board);
                last_winner = board;
            }
        }

        for col in 0..5 {
            if (0..5).all(|row| *at(marked, board, row, col)) {
                winners.insert(board);
                last_winner = board;
            }
        }
    }

    (winners, last_winner)
}
