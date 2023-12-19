use std::collections::VecDeque as Deque;

use utils::{rts, CharGrid, GetOption as _};

fn main() {
    let input = rts(18);
    let input = parse(&input);

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &Vec<Instruction>) {
    let mut grid = Deque::new();
    grid.push_back(Deque::new());
    grid[0].push_back(Some(Line::DR));

    let mut offset = (0, 0);
    let mut pos = (0, 0);
    for (i, instruction) in input.iter().enumerate() {
        for _ in 0..instruction.dist {
            if pos.0 == 0 && instruction.dir == Direction::Left {
                for line in &mut grid {
                    line.push_front(None);
                }
                pos.0 += 1;
                offset.0 += 1;
            }
            if pos.1 == 0 && instruction.dir == Direction::Up {
                grid.push_front(vec![None; grid[0].len()].into());
                pos.1 += 1;
                offset.1 += 1;
            }

            match instruction.dir {
                Direction::Up => pos.1 -= 1,
                Direction::Down => pos.1 += 1,
                Direction::Left => pos.0 -= 1,
                Direction::Right => pos.0 += 1,
            }
            if pos.1 >= grid.len() {
                grid.push_back(vec![None; grid[0].len()].into());
            }
            if pos.0 >= grid[0].len() {
                for row in &mut grid {
                    row.push_back(None);
                }
            }

            grid[pos.1][pos.0] = Some(instruction.dir.into());
        }
        let next = input.get(i + 1).unwrap_or_else(|| &input[0]);
        let line = Line::from_pair(instruction.dir, next.dir);
        grid[pos.1][pos.0] = Some(line);
    }

    for y in 0..grid.len() {
        let mut inside = false;
        let mut start_dir = None;
        for cell in &mut grid[y] {
            match cell {
                None => {
                    if inside {
                        *cell = Some(Line::Inside);
                    }
                }
                Some(Line::Ver) => {
                    inside = !inside;
                }
                Some(Line::DR) => {
                    start_dir = Some(Direction::Down);
                }
                Some(Line::UR) => {
                    start_dir = Some(Direction::Up);
                }
                Some(Line::DL) => {
                    if start_dir == Some(Direction::Up) {
                        inside = !inside;
                    }
                    start_dir = None;
                }
                Some(Line::UL) => {
                    if start_dir == Some(Direction::Down) {
                        inside = !inside;
                    }
                    start_dir = None;
                },
                _ => {}
            }
        }
    }
    let mut sum = 0;
    for row in grid {
        for col in row {
            if col.is_some() {
                sum += 1;
            }
        }
    }
    println!("Result: {}", sum);
}

fn part_2(input: &Vec<Instruction>) {
    // ALL HAIL THE SHOELACE FORMULA :pray: :pray: :pray:

    let mut points = vec![(0, 0)];
    let mut offset = (0, 0);
    let mut edge = 0;

    for instruction in input {
        let dist = i64::from_str_radix(&instruction.color[1..6], 16).unwrap();
        let dir = match &instruction.color[6..] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!()
        };
        let mut new_pos = points.last().unwrap().clone();
        match dir {
            Direction::Up => { new_pos.1 -= dist; offset.1 = offset.1.min(new_pos.1); },
            Direction::Down => new_pos.1 += dist,
            Direction::Left => { new_pos.0 -= dist; offset.0 = offset.0.min(new_pos.0); },
            Direction::Right => new_pos.0 += dist,
        }
        edge += dist;
        points.push(new_pos);
    }

    for point in &mut points {
        point.0 -= offset.0;
        point.1 -= offset.1;
    }

    println!("Result: {}", shoelace(&points) + edge / 2 + 1);
}

fn shoelace(points: &[(i64, i64)]) -> i64 {
    let mut sum = 0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        sum += points[i].0 * points[j].1 - points[j].0 * points[i].1;
    }
    sum.abs() / 2
}

fn parse(input: &String) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            Instruction {
                dir: match parts.next().unwrap() {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!()
                },
                dist: parts.next().unwrap().parse().unwrap(),
                color: parts.next().unwrap()[1..8].to_string()
            }
        }).collect()
}

struct Instruction {
    dir: Direction,
    dist: u64,
    color: String
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Line {
    Ver,
    Hor,
    /// L
    UR,
    UL,
    DR,
    DL,
    Inside
}

impl Line {
    fn from_pair(a: Direction, b: Direction) -> Self {
        match (a, b) {
            (Direction::Up, Direction::Right) => Line::DR,
            (Direction::Up, Direction::Left) => Line::DL,
            (Direction::Down, Direction::Right) => Line::UR,
            (Direction::Down, Direction::Left) => Line::UL,
            (Direction::Right, Direction::Up) => Line::UL,
            (Direction::Right, Direction::Down) => Line::DL,
            (Direction::Left, Direction::Up) => Line::UR,
            (Direction::Left, Direction::Down) => Line::DR,
            _ => panic!()
        
        }
    }
}

impl From<Direction> for Line {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Line::Ver,
            Direction::Down => Line::Ver,
            Direction::Left => Line::Hor,
            Direction::Right => Line::Hor,
        }
    }
}