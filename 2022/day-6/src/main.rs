#![feature(iter_next_chunk)]

use std::{fs::read_to_string, collections::HashSet};

fn no_dupes<const N: usize>(arr: &[char; N]) -> bool {
    let mut set = HashSet::new();
    arr.iter().all(|x| set.insert(x))
}

fn main() {
    // its pretty short no need to buffer
    let signal = read_to_string("input.txt").unwrap();
    let mut chars = signal.chars();
    let mut length = 4;
    let mut recent = chars.next_chunk::<4>().unwrap();
    for c in chars {
        println!("recent: {:?}", recent);
        if no_dupes(&recent) {
            break;
        }
        recent[length % 4] = c;
        length += 1;
    }

    println!("The 4 chaar repeat occurs after {} characters.", length);

    let signal = read_to_string("input.txt").unwrap();
    let mut chars = signal.chars();
    let mut length = 14;
    let mut recent = chars.next_chunk::<14>().unwrap();
    for c in chars {
        println!("recent: {:?}", recent);
        if no_dupes(&recent) {
            break;
        }
        recent[length % 14] = c;
        length += 1;
    }

    println!("The 14 chaar repeat occurs after {} characters.", length);
}
