use std::collections::VecDeque as Deque;

use utils::{rtg, CharGrid, GetOption as _};

fn main() {
    let input = rtg(16);

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &CharGrid) {
    println!("Result: {}", num_energized(&Ray { start: (3,0), dir: Direction::Down }, &input));
}

fn part_2(input: &CharGrid) {
    let down_rays = (0..input.num_cols()).map(|x| Ray {start: (x, 0), dir: Direction::Down});
    let up_rays = (0..input.num_cols()).map(|x| Ray {start: (x, input.num_rows() - 1), dir: Direction::Up});
    let left_rays = (0..input.num_rows()).map(|y| Ray {start: (0, y), dir: Direction::Left});
    let right_rays = (0..input.num_rows()).map(|y| Ray {start: (input.num_cols() - 1, y), dir: Direction::Right});
    let rays = down_rays.chain(up_rays).chain(left_rays).chain(right_rays);
    println!("Result: {}", rays.map(|ray| num_energized(&ray, &input)).max().unwrap());
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ray {
    start: (usize, usize),
    dir: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn num_energized(start_ray: &Ray, grid: &CharGrid) -> u64 {
    let mut energized = CharGrid::new((grid.num_rows(), grid.num_cols()), ' ', ' ');
    let mut queue: Deque<Ray> = Deque::new();
    let mut hist: Vec<Ray> = vec![];
    
    let starts = match start_ray.dir {
        Direction::Up => {
            match grid.get(start_ray.start.0, start_ray.start.1).unwrap() {
                '/' => vec![Ray {start: start_ray.start, dir: Direction::Right}],
                '\\' => vec![Ray {start: start_ray.start, dir: Direction::Left}],
                '-' => vec![
                    Ray {start: start_ray.start, dir: Direction::Left},
                    Ray {start: start_ray.start, dir: Direction::Right},
                ],
                _ => vec![start_ray.clone()]
            }
        },
        Direction::Down => {
            match grid.get(start_ray.start.0, start_ray.start.1).unwrap() {
                '/' => vec![Ray {start: start_ray.start, dir: Direction::Left}],
                '\\' => vec![Ray {start: start_ray.start, dir: Direction::Right}],
                '-' => vec![
                    Ray {start: start_ray.start, dir: Direction::Left},
                    Ray {start: start_ray.start, dir: Direction::Right},
                ],
                _ => vec![start_ray.clone()]
            }
        },
        Direction::Left => {
            match grid.get(start_ray.start.0, start_ray.start.1).unwrap() {
                '/' => vec![Ray {start: start_ray.start, dir: Direction::Down}],
                '\\' => vec![Ray {start: start_ray.start, dir: Direction::Up}],
                '|' => vec![
                    Ray {start: start_ray.start, dir: Direction::Up},
                    Ray {start: start_ray.start, dir: Direction::Down},
                ],
                _ => vec![start_ray.clone()]
            }
        },
        Direction::Right => {
            match grid.get(start_ray.start.0, start_ray.start.1).unwrap() {
                '/' => vec![Ray {start: start_ray.start, dir: Direction::Up}],
                '\\' => vec![Ray {start: start_ray.start, dir: Direction::Down}],
                '|' => vec![
                    Ray {start: start_ray.start, dir: Direction::Up},
                    Ray {start: start_ray.start, dir: Direction::Down},
                ],
                _ => vec![start_ray.clone()]
            }
        }
    };

    queue.extend(starts.clone());
    hist.extend(starts.clone());

    while queue.len() > 0 {
        let ray = queue.pop_front().unwrap();
        let rays = trace(&ray, &grid, &mut energized);
        for ray in rays {
            if !hist.contains(&ray) {
                queue.push_back(ray.clone());
                hist.push(ray.clone());
            }
        }
    }

    let mut sum = 0;
    for y in 0..energized.num_rows() {
        for x in 0..energized.num_cols() {
            if (&energized).get(x, y).unwrap() != ' ' {
                sum += 1;
            }
        }
    }
    sum
}

fn trace(ray: &Ray, grid: &CharGrid, energized: &mut CharGrid) -> Vec<Ray> {
    let mut pos = ray.start;
    loop {
        energized.set(pos.0, pos.1, match ray.dir {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        });
        match ray.dir {
            Direction::Up => if pos.1 > 0 { pos.1 -= 1 } else { return vec![] },
            Direction::Down => pos.1 += 1,
            Direction::Left => if pos.0 > 0 { pos.0 -= 1 } else { return vec![] },
            Direction::Right => pos.0 += 1,
        }
        if let Some(c) = grid.get(pos.0, pos.1) {
            match c {
                '|' => {
                    if ray.dir == Direction::Right || ray.dir == Direction::Left {
                        return vec![
                            Ray {start: pos, dir: Direction::Up},
                            Ray {start: pos, dir: Direction::Down},
                        ]
                    }
                }
                '-' => {
                    if ray.dir == Direction::Up || ray.dir == Direction::Down {
                        return vec![
                            Ray {start: pos, dir: Direction::Left},
                            Ray {start: pos, dir: Direction::Right},
                        ]
                    }
                }
                '/' => {
                    let new_dir = match ray.dir {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                    return vec![Ray {start: pos, dir: new_dir}];
                }
                '\\' => {
                    let new_dir = match ray.dir {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    return vec![Ray {start: pos, dir: new_dir}];
                }
                _ => {}
            }
        } else {
            return vec![];
        }
    }
}