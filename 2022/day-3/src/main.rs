#![feature(iter_array_chunks)]

use std::{fs::File, io::{BufReader, BufRead}};

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 38
    }
    else {
        c as u32 - 96
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0u32;
    for line in reader.lines().map(|x| x.unwrap()) {
        let len = line.len();
        let ruck1 = &line[..len/2];
        let ruck2 = &line[len/2..];
        for c in ruck1.chars() {
            if ruck2.contains(c) {
                sum += priority(c);
                break;
            } 
        }
    }
    println!("The sum of the priorities is {}", sum);

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0u32;
    for group in reader.lines().map(|x| x.unwrap()).array_chunks::<3>() {
        for c in group[0].chars() {
            if group[1].contains(c) && group[2].contains(c) {
                sum += priority(c);
                break;
            }
        }
    }
    println!("The sum of the badge priorities is {}", sum)
}
