use utils::{rts, CharGrid, GetOption as _};

fn main() {
    let input = rts(18);
    let input = parse(&input);

    part_1(&input);
}

fn part_1(input: &Vec<Instruction>) {
    let mut grid = vec![vec![Some(Line::DR)]];
    
    let mut pos = (0, 0);
    for (i, instruction) in input.iter().enumerate() {
        for _ in 0..instruction.dist {
            match instruction.dir {
                Direction::Up => pos.1 -= 1,
                Direction::Down => pos.1 += 1,
                Direction::Left => pos.0 -= 1,
                Direction::Right => pos.0 += 1,
            }
            if pos.1 >= grid.len() {
                grid.push(vec![None; grid[0].len()]);
            }
            if pos.0 >= grid[0].len() {
                for row in &mut grid {
                    row.push(None);
                }
            }
            grid[pos.1][pos.0] = Some(instruction.dir.into());
        }
        if let Some(next) = input.get(i + 1) {
            let line = Line::from_pair(instruction.dir, next.dir);
            grid[pos.1][pos.0] = Some(line);
        }
    }
    grid[0][0] = Some(Line::DR);

    for row in grid {
        for col in row {
            print!("{}", match col {
                Some(Line::Ver) => '|',
                Some(Line::Hor) => '-',
                Some(Line::DL) => '7',
                Some(_) => '*',
                None => ' '
            });
        }
        println!();
    }
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
    DL
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