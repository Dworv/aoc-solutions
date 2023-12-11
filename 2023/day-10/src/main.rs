use utils::{rtgd, CharGrid, GetDefault as _};

const RING: [(isize, isize, Direction); 4] = [
    (0,-1,Direction::North),
    (1,0,Direction::East),
    (0,1,Direction::South),
    (-1,0,Direction::West)
];

fn main() {
    let input = rtgd(10, '.');

    both_parts(&input);
}

fn both_parts(input: &CharGrid) {
    let mut map = CharGrid::new((input.num_cols(), input.num_rows()), ' ', 'X');
    let mut start_dirs = [Direction::North, Direction::North];

    let mut pos = get_start(input);
    let start_pos = pos;
    let mut next_dir = Direction::South;

    for (dx, dy, dir) in RING {
        let c = input.get(pos.0 + dx, pos.1 + dy);
        if let Some(dirs) = char_to_dirs(c) {
            if dirs.contains(&dir.opposite()) {
                next_dir = dir;
                break;
            }
        }
    }

    start_dirs[0] = next_dir;

    let mut tube_length = 0;

    loop {
        tube_length += 1;
        pos = (pos.0 + next_dir.offset().0, pos.1 + next_dir.offset().1);
        let c = input.get(pos.0, pos.1);

        if c == 'S' {
            start_dirs[1] = next_dir.opposite();
            println!("Result: {}", tube_length / 2 + tube_length % 2);
            break;
        }

        if let Some(dirs) = char_to_dirs(c) {
            map.set(pos.0 as usize, pos.1 as usize, c);
            next_dir = dirs.iter().find(|x| **x != next_dir.opposite()).unwrap().clone();
        } else {
            panic!("led to bad char");
        }
    }

    start_dirs.sort();
    map.set(start_pos.0 as usize, start_pos.1 as usize, match start_dirs {
        [Direction::North, Direction::South] => '|',
        [Direction::East, Direction::West] => '-',
        [Direction::North, Direction::East] => 'L',
        [Direction::South, Direction::West] => '7',
        [Direction::North, Direction::West] => 'J',
        [Direction::South, Direction::East] => 'F',
        _ => panic!("bad start dirs")
    });

    let mut contained = 0;
    for y in 0..map.num_rows() {
        let mut inside = false;
        let mut start_dir = None;
        for x in 0..map.num_cols() {
            let c = (&map).get(x as isize, y as isize);
            match c {
                ' ' => {
                    if inside {
                        contained += 1;
                    }
                }
                '|' => {
                    inside = !inside;
                }
                'F' => {
                    start_dir = Some(Direction::South);
                }
                'L' => {
                    start_dir = Some(Direction::North);
                }
                '7' => {
                    if start_dir == Some(Direction::North) {
                        inside = !inside;
                    }
                    start_dir = None;
                }
                'J' => {
                    if start_dir == Some(Direction::South) {
                        inside = !inside;
                    }
                    start_dir = None;
                },
                _ => {}
            }
        }
    }

    println!("Result: {}", contained);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0)
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