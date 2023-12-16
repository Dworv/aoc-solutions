use std::collections::LinkedList;

use utils::rts;

fn main() {
    let input = rts(15);

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &String) {
    let mut sum = 0;
    for step in input.split(',') {
        sum += hash(step);
    }
    println!("Result: {}", sum);
}

fn part_2(input: &String) {
    let mut boxes: Vec<Vec<(&str, u8)>> = vec![vec![]; 256];
    for step in input.split(',') {
        if step.contains('-') {
            let label = step.split('-').nth(0).unwrap();
            let boxnum = hash(label) as usize;
            if let Some(pos) = boxes[boxnum].iter().position(|&(l, _)| l == label) {
                for i in (pos + 1)..boxes[boxnum].len() {
                    let moved = boxes[boxnum][i].clone();
                    boxes[boxnum][i - 1] = moved;
                }
                boxes[boxnum].pop();
            }
        } else {
            let (label, length) = step.split_once('=').unwrap();
            let length = length.parse::<u8>().unwrap();
            let boxnum = hash(label) as usize;
            if let Some(old) = boxes[boxnum].iter_mut().find(|(l, _)| *l == label) {
                old.1 = length;
            } else {
                boxes[boxnum].push((label, length));
            }
        }
    }

    let mut sum = 0;
    for (b, bx) in boxes.iter().enumerate() {
        for (s, &(_, length)) in bx.iter().enumerate() {
            sum += (b + 1) * (s + 1) * length as usize;
        }
    }
    println!("Result: {}", sum);
}

fn hash(s: &str) -> u64 {
    let mut current = 0;
    for c in s.chars() {
        current += c as u8 as u64;
        current *= 17;
        current %= 256;
    }
    current
}