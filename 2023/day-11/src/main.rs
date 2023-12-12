use utils::{rtg, CharGrid, GetOption};

fn main() {
    let input = rtg(11);

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &CharGrid) {
    let galaxies = calc_galaxies(input, 1);

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            sum += galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
        }
    }

    println!("Result: {}", sum);
}

fn part_2(input: &CharGrid) {
    let galaxies = calc_galaxies(input, 999999);

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            sum += galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
        }
    }

    println!("Result: {}", sum);
}

fn calc_galaxies(input: &CharGrid, distance: u64) -> Vec<(u64, u64)> {
    let mut galaxies = vec![];

    let mut empties = 0;
    for y in 0..input.num_rows() {
        let mut is_empty = true;
        for x in 0..input.num_cols() {
            if input.get(x, y).unwrap() == '#' {
                is_empty = false;
                galaxies.push((x as u64, y as u64 + empties * distance))
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

        galaxy.0 += empties as u64 * distance;
    }

    galaxies
}