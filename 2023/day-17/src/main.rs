use std::collections::VecDeque as Deque;

use utils::{rtg, CharGrid, GetOption as _};

fn main() {
    let input = rtg(17);

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &CharGrid) {
    let mut cost_graph = vec![vec![[[999999999; 3]; 4]; input.num_rows()]; input.num_cols()];
    let mut queue: Deque<State> = Deque::new();

    cost_graph[0][0] = [[0; 3]; 4];
    cost_graph[0][1] = [[input.get(0, 1).unwrap().to_digit(10).unwrap(); 3]; 4];
    cost_graph[1][0] = [[input.get(1, 0).unwrap().to_digit(10).unwrap(); 3]; 4];

    queue.push_back(State {
        pos: (0, 1),
        dir: Direction::Down,
        streak: 1,
    });
    queue.push_back(State {
        pos: (1, 0),
        dir: Direction::Right,
        streak: 1,
    });

    while let Some(state) = queue.pop_front() {
        use Direction::*;
        let (x, y) = state.pos;
        let neighbors: Vec<(usize, usize, Direction)> =
            vec![(0, 1, Down), (1, 0, Right), (0, -1, Up), (-1, 0, Left)]
                .iter()
                .filter_map(|(dx, dy, d)| {
                    if (state.dir == *d && state.streak >= 3)
                        || state.dir.opposite() == *d
                        || x as isize + dx < 0
                        || y as isize + dy < 0
                        || x as isize + dx >= input.num_cols() as isize
                        || y as isize + dy >= input.num_rows() as isize
                    {
                        None
                    } else {
                        Some(((x as isize + dx) as usize, (y as isize + dy) as usize, *d))
                    }
                })
                .collect();

        for (nx, ny, d) in neighbors {
            let nstreak = if state.dir == d { state.streak + 1 } else { 1 };
            let nstate = State {
                pos: (nx, ny),
                dir: d,
                streak: nstreak,
            };
            // dbg!(&nstate);
            let ncost = cost_graph[x][y][state.dir as usize][state.streak as usize - 1] + input.get(nx, ny).unwrap().to_digit(10).unwrap();
            // dbg!(&ncost);

            if cost_graph[nx][ny][d as usize][nstreak as usize - 1] > ncost {
                cost_graph[nx][ny][d as usize][nstreak as usize - 1] = ncost;
                queue.push_back(nstate);
            }
        }
    }

    println!(
        "Result: {}",
        cost_graph[input.num_cols() - 1][input.num_rows() - 1]
            .iter()
            .flat_map(|x| x.iter())
            .min()
            .unwrap()
    );
}

fn part_2(input: &CharGrid) {
    let mut cost_graph = vec![vec![[[999999999; 10]; 4]; input.num_rows()]; input.num_cols()];
    let mut queue: Deque<State> = Deque::new();

    cost_graph[0][0] = [[0; 10]; 4];
    cost_graph[0][1] = [[input.get(0, 1).unwrap().to_digit(10).unwrap(); 10]; 4];
    cost_graph[1][0] = [[input.get(1, 0).unwrap().to_digit(10).unwrap(); 10]; 4];

    queue.push_back(State {
        pos: (0, 1),
        dir: Direction::Down,
        streak: 1,
    });
    queue.push_back(State {
        pos: (1, 0),
        dir: Direction::Right,
        streak: 1,
    });

    while let Some(state) = queue.pop_front() {
        use Direction::*;
        let (x, y) = state.pos;
        let neighbors: Vec<(usize, usize, Direction)> =
            vec![(0, 1, Down), (1, 0, Right), (0, -1, Up), (-1, 0, Left)]
                .iter()
                .filter_map(|(dx, dy, d)| {
                    if (state.dir == *d && state.streak >= 10)
                        || state.dir != *d && state.streak < 4
                        || state.dir.opposite() == *d
                        || x as isize + dx < 0
                        || y as isize + dy < 0
                        || x as isize + dx >= input.num_cols() as isize
                        || y as isize + dy >= input.num_rows() as isize
                    {
                        None
                    } else {
                        Some(((x as isize + dx) as usize, (y as isize + dy) as usize, *d))
                    }
                })
                .collect();

        for (nx, ny, d) in neighbors {
            let nstreak = if state.dir == d { state.streak + 1 } else { 1 };
            let nstate = State {
                pos: (nx, ny),
                dir: d,
                streak: nstreak,
            };
            // dbg!(&nstate);
            let ncost = cost_graph[x][y][state.dir as usize][state.streak as usize - 1] + input.get(nx, ny).unwrap().to_digit(10).unwrap();
            // dbg!(&ncost);

            if cost_graph[nx][ny][d as usize][nstreak as usize - 1] > ncost {
                cost_graph[nx][ny][d as usize][nstreak as usize - 1] = ncost;
                queue.push_back(nstate);
            }
        }
    }

    println!(
        "Result: {}",
        cost_graph[input.num_cols() - 1][input.num_rows() - 1]
            .iter()
            .flat_map(|x| x.iter())
            .min()
            .unwrap()
    );
}

#[derive(Debug, Clone, Copy)]
struct State {
    pos: (usize, usize),
    dir: Direction,
    streak: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}