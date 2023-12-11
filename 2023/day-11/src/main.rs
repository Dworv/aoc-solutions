use utils::{rtg, CharGrid, GetOption};

fn main() {
    let input = rtg(11);

    part_1(&input);
}

fn part_1(input: &CharGrid) {
    let galaxies = calc_galaxies(input);

    dbg!(galaxies);
}

fn calc_galaxies(input: &CharGrid) -> Vec<(u64, u64)> {
    let mut galaxies = vec![];

    let mut empties = 0;
    for y in 0..input.num_rows() {
        let mut is_empty = true;
        for x in 0..input.num_cols() {
            if input.get(x, y).unwrap() == '#' {
                is_empty = false;
                galaxies.push((x as u64, y as u64 + empties))
            }
        }

        if is_empty {
            empties += 1;
        }
    }

    galaxies.sort_unstable_by_key(|x| x.0);
    let mut empties = 0;
    let mut last = -1;

    for galaxy in &mut galaxies {
        if last != galaxy.0 as i64 {
            empties += galaxy.0 as i64 - (last + 1);
            last = galaxy.0 as i64;
        }

        galaxy.0 += empties as u64;
    }

    galaxies
}