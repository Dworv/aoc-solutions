use utils::{rts, CharGrid, GetDefault as _};

fn main() {
    let input = rts(13);
    let input = parse(&input);

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &Vec<CharGrid>) {
    let mut sum = 0;
    for grid in input {
        let veq_matrix = veqs(&grid);
        sum += to_refls(&veq_matrix);

        let heq_matrix = heqs(&grid);
        sum += to_refls(&heq_matrix) * 100;
    }
    println!("Result: {}", sum);
}

fn part_2(input: &Vec<CharGrid>) {
    let mut sum = 0;
    for grid in input {
        let veq_matrix = veqs(&grid);
        sum += to_smudged(&veq_matrix);

        let heq_matrix = heqs(&grid);
        sum += to_smudged(&heq_matrix) * 100;
    }
    println!("Result: {}", sum);
}

fn to_refls(matrix: &CharGrid) -> u64 {
    let mut refl_index_sum = 0;
    for i in 1..matrix.num_rows() {
        let i = i as isize;
        let mut pos = (i, i-1);
        let mut has_reflection = true;
        while pos.0 < matrix.num_cols() as isize && pos.1 >= 0 {
            if (&matrix).get(pos.0, pos.1) != 'X' {
                has_reflection = false;
                break;
            }
            pos.0 += 1;
            pos.1 -= 1;
        }
        if has_reflection {
            refl_index_sum += i as u64;
        }
    }
    refl_index_sum
}

fn to_smudged(matrix: &CharGrid) -> u64 {
    let mut smudged_index_sum = 0;
    for i in 1..matrix.num_rows() {
        let i = i as isize;
        let mut pos = (i, i-1);
        let mut has_smudge = false;
        while pos.0 < matrix.num_cols() as isize && pos.1 >= 0 {
            match (&matrix).get(pos.0, pos.1) {
                'O' => {
                    if has_smudge {
                        has_smudge = false;
                        break;
                    }
                    has_smudge = true;
                },
                ' ' => { has_smudge = false; break},
                _ => {}
            }
            pos.0 += 1;
            pos.1 -= 1;
        }
        if has_smudge {
            smudged_index_sum += i as u64;
        }
    }
    smudged_index_sum
}

fn veqs(grid: &CharGrid) -> CharGrid {
    let mut veq_matrix = CharGrid::new((grid.num_cols(), grid.num_cols()), ' ', ' ');
    for i in 0..grid.num_cols() {
        for j in (i + 1)..grid.num_cols() {
            let c1 = grid.col(i);
            let c2 = grid.col(j);
            veq_matrix.set(j, i, match diffs(c1, c2) {
                0 => 'X',
                1 => 'O',
                2.. => ' '
            });
        }
    }
    veq_matrix
}

fn heqs(grid: &CharGrid) -> CharGrid {
    let mut heq_matrix = CharGrid::new((grid.num_rows(), grid.num_rows()), ' ', ' ');
    for i in 0..grid.num_rows() {
        for j in (i + 1)..grid.num_rows() {
            let c1 = grid.row(i);
            let c2 = grid.row(j);
            heq_matrix.set(j, i, match diffs(c1, c2) {
                0 => 'X',
                1 => 'O',
                2.. => ' '
            });
        }
    }
    heq_matrix
}

fn diffs<T: Iterator<Item = char>>(a: T, b: T) -> u64 {
    a.zip(b).fold(0, |acc, (a, b)| acc + (a != b) as u64)
}

fn parse(input: &String) -> Vec<CharGrid> {
    let mut grids = vec![];
    let mut cgrid = String::new();
    for line in input.lines() {
        if line.is_empty() {
            grids.push(CharGrid::from_str(&cgrid, ' '));
            cgrid.clear();
        } else {
            if !cgrid.is_empty() { cgrid.push('\n'); }
            cgrid.push_str(line);
        }
    }
    grids.push(CharGrid::from_str(&cgrid, ' '));
    grids
}
