use utils::{rtg, CharGrid};

fn main() {
    let input = rtg(3);
    part_1(&input);
}

fn part_1(input: &CharGrid) {
    let mut sum = 0;
    for y in 0..input.num_rows() {
        let mut raw_num = String::new();
        for (x, c) in input.row(y).enumerate() {
            if c.is_digit(10) {
                raw_num.push(c);
            } else if !raw_num.is_empty() {
                if has_part(&input, y, x-raw_num.len(), x-1) {
                    sum += raw_num.parse::<usize>().unwrap();
                }
                raw_num.clear();
            }
        }
        if !raw_num.is_empty() {
            if has_part(&input, y, input.num_cols()-raw_num.len(), input.num_cols()-1) {
                sum += raw_num.parse::<usize>().unwrap();
            }
        }
    }
    println!("{}", sum);
}

fn has_part(input: &CharGrid, y: usize, start: usize, end: usize) -> bool {
    let left_edge = start.saturating_sub(1);
    let right_edge = end.saturating_add(1);
    let mut edges = vec![
        input.get(left_edge, y),
        input.get(end.saturating_add(1), y),
    ];
    for x in left_edge..=right_edge {
        edges.push(input.get(x, y.saturating_sub(1)));
        edges.push(input.get(x, y.saturating_add(1)));
    }
    for c in edges {
        if let Some(c) = c {
            if !c.is_digit(10) && c != '.' {
                return true
            }
        }
    }
    false
}
