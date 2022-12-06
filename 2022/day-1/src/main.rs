use std::{fs::File, io::{BufReader, prelude::*}};

fn main() {
    let file = File::open("input.txt").expect("no file");
    let reader = BufReader::new(file);
    
    let mut best = 0;
    let mut current = 0;
    for line in reader.lines().map(|x| x.unwrap()) {
        match line.parse::<u32>() {
            Ok(cals) => {
                current += cals
            }
            Err(_) => {
                best = best.max(current);
                current = 0;
            }
        }
    }
    
    println!("The biggest collection of calories is {}", best);
    
    let file = File::open("input.txt").expect("no file");
    let reader = BufReader::new(file);
    
    let mut best = [0; 3];
    let mut current = 0;
    for line in reader.lines().map(|x| x.unwrap()) {
        match line.parse::<u32>() {
            Ok(cals) => {
                current += cals;
            }
            Err(_) => {
                best.sort();
                if current > best[0] {
                    best[0] = current;
                }
                current = 0;
            }
        }
    }
    
    println!("The top 3 elves have {} calories.", best.iter().fold(0, |a, x| a + x))
}
