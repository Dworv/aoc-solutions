use utils::{rtg, CharGrid, GetOption as _};

use rand::{thread_rng, Rng};

fn main() {
    let input = rtg(14);

    part_1(&mut input.clone());
    part_2(&mut input.clone());
}

fn part_1(input: &mut CharGrid) {
    for x in 0..input.num_cols() {
        let mut open_index = 0;

        for y in 0..input.num_rows() {
            match (&*input).get(x, y).unwrap() {
                '#' => {
                    open_index = y + 1;
                }
                'O' => {
                    input.set(x, y, '.');
                    input.set(x, open_index, 'O');
                    open_index += 1;
                }
                _ => {}
            }
        }
    }

    let sum = n_weight(input);

    println!("Result: {}", sum);
}

fn part_2(input: &mut CharGrid) {
    let mut history = vec![input.clone()];
    let mut i = 0;
    let old_match = loop {
        i += 4;
        shift_north(input);
        shift_west(input);
        shift_south(input);
        shift_east(input);
        if let Some(prev) = history.iter().enumerate().find_map(|(p, g)| if g == input { Some(p * 4) } else { None }) {
            break prev;
        }
        history.push(input.clone());
    };

    let gap = i - old_match;
    let remainder = (4_000_000_000 - old_match) % gap;
    dbg!(gap, i, old_match, remainder);
    for _ in 0..(remainder/4) {
        shift_north(input);
        shift_west(input);
        shift_south(input);
        shift_east(input);
    }

    let sum = n_weight(input);

    // let mut input = history[0].clone();
    // for _ in 0..10000/4 {
    //     shift_north(&mut input);
    //     shift_west(&mut input);
    //     shift_south(&mut input);
    //     shift_east(&mut input);
    // }

    // dbg!(n_weight(&input));

    println!("Result: {}", sum);
}

fn n_weight(input: &CharGrid) -> u64 {
    let mut sum = 0;
    for y in 0..input.num_rows() {
        for c in input.row(y) {
            if c == 'O' {
                sum += input.num_rows() - y;
            }
        }
    }
    sum as u64
}

fn shift_north(input: &mut CharGrid) {
    for x in 0..input.num_cols() {
        let mut open_index = 0;

        for y in 0..input.num_rows() {
            match (&*input).get(x, y).unwrap() {
                '#' => {
                    open_index = y + 1;
                }
                'O' => {
                    input.set(x, y, '.');
                    input.set(x, open_index, 'O');
                    open_index += 1;
                }
                _ => {}
            }
        }
    }
}

fn shift_south(input: &mut CharGrid) {
    for x in 0..input.num_cols() {
        let mut open_index = input.num_rows() - 1;

        for y in (0..input.num_rows()).rev() {
            match (&*input).get(x, y).unwrap() {
                '#' => {
                    open_index = y.saturating_sub(1);
                }
                'O' => {
                    input.set(x, y, '.');
                    input.set(x, open_index, 'O');
                    open_index = open_index.saturating_sub(1);
                }
                _ => {}
            }
        }
    }
}

fn shift_east(input: &mut CharGrid) {
    for y in 0..input.num_rows() {
        let mut open_index = input.num_cols() - 1;

        for x in (0..input.num_cols()).rev() {
            match (&*input).get(x, y).unwrap() {
                '#' => {
                    open_index = x.saturating_sub(1);
                }
                'O' => {
                    input.set(x, y, '.');
                    input.set(open_index, y, 'O');
                    open_index = open_index.saturating_sub(1);
                }
                _ => {}
            }
        }
    }
}

fn shift_west(input: &mut CharGrid) {
    for y in 0..input.num_rows() {
        let mut open_index = 0;

        for x in 0..input.num_cols() {
            match (&*input).get(x, y).unwrap() {
                '#' => {
                    open_index = x + 1;
                }
                'O' => {
                    input.set(x, y, '.');
                    input.set(open_index, y, 'O');
                    open_index += 1;
                }
                _ => {}
            }
        }
    }
}