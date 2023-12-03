use utils::oldgrid::{rtg, CharGrid};

use std::collections::VecDeque as Deque;

fn main() {
    let input = rtg(3);
    part_1(&input);
    part_2(&input);
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

fn part_2(input: &CharGrid) {
    let mut sum = 0;
    for y in 0..input.num_rows() {
        for (x, c) in input.row(y).enumerate() {
            if c == '*' {
                let nums = adjacent_nums(&input, x, y);
                if nums.len() > 1 {
                    sum += nums.iter().product::<u32>();
                }
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

fn adjacent_nums(input: &CharGrid, x: usize, y: usize) -> Vec<u32> {
    let mut res = Vec::new();
    let on_left_edge = x == 0;
    let on_right_edge = x == input.num_cols() - 1;
    let on_top_edge = y == 0;
    let on_bottom_edge = y == input.num_rows() - 1;
    if !on_left_edge {
        if let Some(num) = scan_number(input, x - 1 , y) {
            res.push(num);
        }
    }
    if !on_right_edge {
        if let Some(num) = scan_number(input, x + 1, y) {
            res.push(num);
        }
    }
    if !on_top_edge {
        res.extend(scan_triple(input, x, y - 1, on_left_edge, on_right_edge));
    }
    if !on_bottom_edge {
        res.extend(scan_triple(input, x, y + 1, on_left_edge, on_right_edge));
    }
    res
}

fn scan_triple(input: &CharGrid, x: usize, y: usize, on_left_edge: bool, on_right_edge: bool) -> Vec<u32> {
    if !on_left_edge && !on_right_edge && !input.get(x, y).unwrap().is_digit(10) {
        [x-1, x+1].iter().filter_map(|x| scan_number(input, *x, y)).collect()
    } else {
        for i in 0..=2 {
            if let Some(x) = (x + i).checked_sub(1) {
                if x < input.num_cols() {
                    if let Some(num) = scan_number(input, x, y) {
                        return vec![num]
                    }
                }
            };
        }
        Vec::new()
    }
}

fn scan_number(input: &CharGrid, x: usize, y: usize) -> Option<u32> {
    let mut collector = Deque::new();
    let origin = input.get(x, y).unwrap();
    if origin.is_digit(10) {
        collector.push_back(origin);
    } else {
        return None
    }
    let mut l = 1;
    while x.checked_sub(l).is_some() && input.get(x-l, y).unwrap().is_digit(10) {
        collector.push_front(input.get(x-l, y).unwrap());
        l += 1;
    }
    let mut r = 1;
    while x+r < input.num_cols() && input.get(x+r, y).unwrap().is_digit(10) {
        collector.push_back(input.get(x+r, y).unwrap());
        r += 1;
    }

    Some(collector.iter().collect::<String>().parse().unwrap())
}