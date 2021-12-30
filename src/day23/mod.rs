use std::collections::{HashSet, BTreeMap, HashMap};

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

type Pos = (i16, i16);
type Board = BTreeMap<Pos, char>;

fn star1(input: String) -> i128 {
    let board = parse_input(&input);
    shortest_path(neighbours, zero_heuristic, board, goal(2), 2).unwrap()
}

fn zero_heuristic(board: &Board, room_size: usize) -> i128 {
    0
}

fn parse_input(input: &str) -> Board {
    input.split("\n").enumerate()
        .flat_map(|(y, line)|
            line.chars().enumerate()
                .map(move |(x, c)| ((x as i16, y as i16), c))
        )
        .filter(|(_, c)| ['.', 'A', 'B', 'C', 'D'].contains(c))
        .collect()
}

fn goal(size: usize) -> Board {
    let mut input: String = 
        "#############\n\
        #...........#\n\
        ###A#B#C#D###\n".to_string();

    for _ in 0..(size - 1) {
        input.push_str("  #A#B#C#D#\n");
    }

    parse_input(input.as_str())
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Eq, PartialEq)]
struct State {
    g: i128,
    f: i128,
    position: Board,
    previous_mover: Pos,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(
    neighbours_fn: fn (&Board, Pos, i16) -> Vec<(Board, i128, Pos)>, 
    h: fn (&Board, usize) -> i128,
    start: Board, 
    goal: Board,
    room_size: i16,
) -> Option<i128> {
    println!("Start: ");
    display(&start);
    println!("Goal: ");
    display(&goal);

    let mut open = BinaryHeap::new();
    open.push(State {
        g: 0, 
        f: h(&start, room_size as usize), 
        position: start.clone(), 
        previous_mover: (-1, -1) }
    );
    let mut dist = hashmap!{ start => 0 };

    let mut count = 0;
    while let Some(State { g, f, position, previous_mover }) = open.pop() {
        count += 1;

        if position == goal { 
            println!("Visited nodes: {}", count);
            return Some(g); 
        }

        if g > dist[&position] { continue; }

        for (n, n_cost, mover) in neighbours_fn(&position, previous_mover, room_size) {
            // println!("neighbour with cost {}: ", n_cost);
            // display(&n);
            let tentative_g = g + n_cost;

            if !dist.contains_key(&n) || tentative_g < *dist.get(&n).unwrap() {
                //println!("Best path so far, total cost is {}, updating", tentative_g);
                let f = tentative_g + h(&n, room_size as usize);
                let next = State { g: tentative_g, f, position: n, previous_mover: mover };
                dist.insert(next.position.clone(), tentative_g);
                open.push(next);
            } else {
                //println!("Total cost {} is worse than best path, ignoring", tentative_g);
            }
        }
    }

    None
}

fn neighbours(board: &Board, previous_mover: Pos, room_size: i16) -> Vec<(Board, i128, Pos)> {
    let mut result = Vec::new();

    let spaces_outside_rooms = [(3, 1), (5, 1), (7, 1), (9, 1)];
    let rooms: HashMap<char, Vec<_>> = hashmap!{
        'A' => (2..(2 + room_size)).map(|y| (3, y)).collect(),
        'B' => (2..(2 + room_size)).map(|y| (5, y)).collect(),
        'C' => (2..(2 + room_size)).map(|y| (7, y)).collect(),
        'D' => (2..(2 + room_size)).map(|y| (9, y)).collect(),
    };
    let costs = hashmap!{
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
    };
    let hallway = [
        (1, 1),
        (2, 1),
        (4, 1),
        (6, 1),
        (8, 1),
        (10, 1),
        (11, 1),
    ];

    // CHeck if any amphipod occupies a space outside a room. 
    // If so, move only that one.
    let amphipod_outside_room = board.iter()
        .filter(|(_, c)| **c != '.')
        .filter(|(pos, _)| spaces_outside_rooms.contains(pos))
        .next();

    for (pos, c) in board.iter() {
        if *c == '.' { continue; }

        if let Some((aor_pos, _)) = amphipod_outside_room {
            if pos != aor_pos { continue; }
        }

        let (x, y) = *pos;
        for next in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if !board.contains_key(&next) { continue; }
            if board[&next] != '.' { continue; }

            // If current position is a hallway AND
            // next is in a room, check that
            // 1. It's the destination room for that amphipod type
            // 2. It doesn't contain amphipods of other types
            if y == 1 && rooms.values().any(|r| r.contains(&next)) {
                if !rooms[c].contains(&next) { continue; }
                if ![*c, '.'].contains(&board[&rooms[c][1]]) { continue }
            }

            // If current position is a hallway position,
            // ensure that amphipod has a clear path to a legal
            // room position, unless it's the previous mover
            if *pos != previous_mover && hallway.contains(&pos) && !has_clear_path(&board, *pos, rooms[c][0]) {
                continue;
            }

            // If above conditions are met, create a new board
            // position.
            let mut new_board = board.clone();
            new_board.insert(*pos, '.');
            new_board.insert(next, *c);
            result.push((new_board, costs[c], next));
        }
    }

    result
}

fn has_clear_path(board: &Board, start: Pos, end: Pos) -> bool {
    let mut open = hashset!{start};
    let mut closed = HashSet::new();

    while !open.is_empty() {
        let (x, y) = *open.iter().next().unwrap();
        open.remove(&(x, y));
        let neighbours: Vec<_> = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter()
            .filter(|pos| board.contains_key(&pos) && board[&pos] == '.')
            .collect();

        for n in neighbours.into_iter() {
            if n == end {
                return true;
            }

            if closed.contains(&n) {
                continue;
            }

            open.insert(n);
        }

        closed.insert((x, y));
    }

    false
}

fn display(board: &Board) {
    let max_y = board.keys().map(|(_, y)| *y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..13 {
            if board.contains_key(&(x, y)) {
                print!("{}", board[&(x, y)]);
            } else {
                print!("#");
            }
        }
        println!();
    }
    println!();
}

fn heuristic(board: &Board, room_size: usize) -> i128 {
    let targets: HashMap<char, Vec<_>> = hashmap!{
        'A' => (0..room_size).map(|i| (3 as i16, 2 + i as i16)).collect(),
        'B' => (0..room_size).map(|i| (5 as i16, 2 + i as i16)).collect(),
        'C' => (0..room_size).map(|i| (7 as i16, 2 + i as i16)).collect(),
        'D' => (0..room_size).map(|i| (9 as i16, 2 + i as i16)).collect(),
    };

    let mut marked = hashset!{};

    // Set result sum to 0
    let mut result = 0;

    // Go through each target position, starting with
    // innermost position in each room
    for i in (0..room_size).rev() {
        for c in "ABCD".chars() {
            let target: (i16, i16) = targets[&c][i];

            // Flood fill from target position, terminating
            // when finding closest applicable amphipod
            // that is not already marked
            let mut open = hashset!{target};
            let mut closed = hashset!{};
            let mut dist = hashmap!{target => 0};
            while !open.is_empty() {
                let current = *open.iter().next().unwrap();
                open.remove(&current);
                closed.insert(current);
                let cur_c = board[&current];

                if cur_c == c && !marked.contains(&current) {
                    result += dist[&current];
                    marked.insert(current);
                    break;
                }

                // Find neighbours
                let (x, y) = current;
                let adjacent: Vec<_> = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].into_iter()
                    .filter(|pos| board.contains_key(&pos) && !closed.contains(&pos))
                    .collect();

                for n in adjacent {
                    dist.insert(n, dist[&current] + 1);
                    open.insert(n);
                }
            }
        }
    }

    // Return result sum
    result
}


fn star2(input: String) -> i128 {
    let board = parse_input(&input);
    shortest_path(neighbours, heuristic, board, goal(4), 4).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input = 
            "#############\n\
            #..A........#\n\
            ###.#B#C#D###\n\
            ###A#B#C#D#";

        assert_map(input, 1, 2);
    }

    #[test]
    fn test_simple2() {
        let input = 
            "#############\n\
            #....B......#\n\
            ###A#.#C#D###\n\
            ###A#B#C#D#";

        assert_map(input, 10, 2);
    }

    #[test]
    fn wont_move_into_wrong_room() {
        let start = 
            "#############\n\
            #CC....B....#\n\
            ###A#.#.#D###\n\
            ###A#B#.#D#";

        let neighbour = 
            "#############\n\
            #CC.........#\n\
            ###A#.#B#D###\n\
            ###A#B#.#D#";

        assert_eq!(is_neighbour(start, neighbour, (7, 1), 2), false);
    }

    #[test]
    fn will_move_with_clear_path() {
        let start = 
            "#############\n\
            #...B......C#\n\
            ###A#.#.#D###\n\
            ###A#B#C#D#";

        let neighbour = 
            "#############\n\
            #...B.....C.#\n\
            ###A#.#.#D###\n\
            ###A#B#C#D#";

        assert_eq!(is_neighbour(start, neighbour, (12, 1), 2), true);
    }

    #[test]
    fn test_example_path() {
        let start = 
            "#############\n\
            #...........#\n\
            ###B#C#B#D###\n\
            ###A#D#C#A#";

        let neighbour = 
            "#############\n\
            #......B....#\n\
            ###B#C#.#D###\n\
            ###A#D#C#A#";
        assert_eq!(is_neighbour(start, neighbour, (-1, -1), 2), true);

        let start = neighbour;
        let neighbour = 
            "#############\n\
            #.....B.....#\n\
            ###B#C#.#D###\n\
            ###A#D#C#A#";
        assert_eq!(is_neighbour(start, neighbour, (7, 1), 2), true);

        let start = neighbour;
        let neighbour = 
            "#############\n\
            #....B......#\n\
            ###B#C#.#D###\n\
            ###A#D#C#A#";
        assert_eq!(is_neighbour(start, neighbour, (6, 1), 2), true);

        let start = neighbour;
        let neighbour = 
            "#############\n\
            #...B.......#\n\
            ###B#C#.#D###\n\
            ###A#D#C#A#";
        assert_eq!(is_neighbour(start, neighbour, (5, 1), 2), true);

        let start = neighbour;
        let neighbour = 
            "#############\n\
            #...BC......#\n\
            ###B#.#.#D###\n\
            ###A#D#C#A#";
        assert_eq!(is_neighbour(start, neighbour, (4, 1), 2), true);

        let start = neighbour;
        let neighbour = 
            "#############\n\
            #...B.C.....#\n\
            ###B#.#.#D###\n\
            ###A#D#C#A#";
        assert_eq!(is_neighbour(start, neighbour, (5, 1), 2), true);

        let start = neighbour;
        let neighbour = 
            "#############\n\
            #...B..C....#\n\
            ###B#.#.#D###\n\
            ###A#D#C#A#";
        assert_eq!(is_neighbour(start, neighbour, (6, 1), 2), true);

        let start = neighbour;
        let neighbour = 
            "#############\n\
            #...B.......#\n\
            ###B#.#C#D###\n\
            ###A#D#C#A#";
        assert_eq!(is_neighbour(start, neighbour, (7, 1), 2), true);

        let start = neighbour;
        let neighbour = 
            "#############\n\
            #...B.......#\n\
            ###B#D#C#D###\n\
            ###A#.#C#A#";
        assert_eq!(is_neighbour(start, neighbour, (7, 2), 2), true);
    }

    #[test]
    fn test_move_up_inside_room() {
        let start = 
            "#############\n\
            #...B.......#\n\
            ###B#.#C#D###\n\
            ###A#D#C#A#";
        let neighbour = 
            "#############\n\
            #...B.......#\n\
            ###B#D#C#D###\n\
            ###A#.#C#A#";
        assert_eq!(is_neighbour(start, neighbour, (7, 2), 2), true);
    }

    #[test]
    fn test_size_3() {
        let input = 
            "#############\n\
            #..A........#\n\
            ###.#B#C#D###\n\
            ###A#B#C#D#\n\
            ###A#B#C#D#";

        assert_map(input, 1, 3);
    }

    fn assert_map(input: &str, expected_score: i128, room_size: i16) {
        let board = parse_input(input);
        let result = shortest_path(neighbours, zero_heuristic, board, goal(room_size as usize), room_size);
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), expected_score);
    }

    fn is_neighbour(board: &str, neighbour: &str, mover: Pos, room_size: i16) -> bool {
        let board = parse_input(board);
        let neighbour = parse_input(neighbour);

        neighbours(&board, mover, room_size)
            .iter()
            .any(|(n, _, _)| n == &neighbour)
    }
}
