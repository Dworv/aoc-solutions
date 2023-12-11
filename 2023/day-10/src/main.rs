use utils::{rtgd, CharGrid, GetDefault as _};

const RING: [(isize, isize, Direction); 4] = [
    (0,-1,Direction::North),
    (1,0,Direction::East),
    (0,1,Direction::South),
    (-1,0,Direction::West)
];

fn main() {
    let input = rtgd(10, '.');

    part_1(&input);
}

fn part_1(input: &CharGrid) {
    let start_pos = get_start(input);
    let mut pos = (-1, -1);

    for (dx, dy, dir) in RING {
        let c = input.get(start_pos.0 + dx, start_pos.1 + dy);
        if let Some(dirs) = char_to_dirs(c) {
            if dirs.contains(&dir.opposite()) {
                pos = (start_pos.0 + dx, start_pos.1 + dy);
                break;
            }
        }
    }

    let mut tube_length = 1;
    let mut last_pos = start_pos;

    loop {
        for (dx, dy, dir) in RING {
            if (pos.0 + dx, pos.1 + dy) == last_pos {
                continue;
            }
            let c = input.get(pos.0 + dx, pos.1 + dy);

            if c == 'S' {
                println!("Result: {}", tube_length / 2 + tube_length % 2);
                return;
            }

            if let Some(dirs) = char_to_dirs(c) {
                if dirs.contains(&dir.opposite()) {
                    last_pos = pos;
                    pos = (pos.0 + dx, pos.1 + dy);
                    tube_length += 1;
                    break;
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North, South, East, West
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East
        }
    }
}

fn char_to_dirs(c: char) -> Option<[Direction; 2]> {
    match c {
        '|' => Some([Direction::North, Direction::South]),
        '-' => Some([Direction::East, Direction::West]),
        'L' => Some([Direction::North, Direction::East]),
        '7' => Some([Direction::South, Direction::West]),
        'J' => Some([Direction::North, Direction::West]),
        'F' => Some([Direction::South, Direction::East]),
        _ => None
    }
}

fn get_start(input: &CharGrid) -> (isize, isize) {
    for y in 0..input.num_rows() {
        for x in 0..input.num_cols() {
            if input.get(x as isize, y as isize) == 'S' {
                return (x as isize, y as isize);
            }
        }
    };
    (-1, -1)
}