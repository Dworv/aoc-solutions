use std::{collections::VecDeque as Deque, fs::File, io::{BufRead, BufReader}};

type Stacks = Vec<Deque<char>>;

trait VecExt<T> {
    fn get_mut_fill_defaults(&mut self, index: usize) -> &mut T;
}

/// i kind of want to write an rfc for this
impl<T: Default> VecExt<T> for Vec<T> {
    fn get_mut_fill_defaults(&mut self, index: usize) -> &mut T {
        let len = self.len();
        if let Some(amt) = index.checked_sub(len) {
            for _ in 0..=amt {
                self.push(Default::default());
            }
        }
        unsafe { self.get_unchecked_mut(index) } // its safe because we literally just filled it
    }
}

fn gen_stacks(lines: &mut dyn Iterator<Item = String>) -> Stacks {
    let mut stacks: Stacks = Default::default();
    
    'outer: loop {
        let line = lines.next().unwrap();
        let chars = line.chars().skip(1).step_by(4);
        for (i, c) in chars.enumerate() {
            if "0123456789".contains(c) {
                break 'outer
            }
            if c != ' ' {
                match stacks.get_mut_fill_defaults(i) {
                    deque => {
                        deque.push_back(c)
                    }
                }
            }
        }
    }
    stacks
}

fn next_u32(chars: &mut dyn Iterator<Item = char>) -> Option<u32> {
    let mut digits = String::new();
    loop {
        if let Some(c) = chars.next() {
            if "0123456789".contains(c) {
                digits.push(c)
            } else if digits.len() != 0 {
                return Some(digits.parse().unwrap());
            }
        } else {
            if digits.len() > 0 {
                return Some(digits.parse().unwrap());
            } else {
                return None
            }
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|x| x.unwrap());

    let mut stacks = gen_stacks(&mut lines);

    lines.next(); // skip blank line

    for line in lines {
        let mut chars = line.chars();
        let amt = next_u32(&mut chars).unwrap();
        let start = next_u32(&mut chars).unwrap();
        let end = next_u32(&mut chars).unwrap();

        for _ in 0..amt {
            let lifted = stacks[start as usize - 1].pop_front().unwrap();
            stacks[end as usize - 1].push_front(lifted);
        }
    }

    let mut tops = String::new();
    for stack in &mut stacks {
        tops.push(stack.pop_front().unwrap())
    }

    println!("The top crates for the cratemover 9000 are {}.", tops);

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|x| x.unwrap());

    let mut stacks = gen_stacks(&mut lines);

    lines.next(); // skip blank line

    for line in lines {
        let mut chars = line.chars();
        let amt = next_u32(&mut chars).unwrap();
        let start = next_u32(&mut chars).unwrap();
        let end = next_u32(&mut chars).unwrap();
        let mut tmp = Vec::new();
        for _ in 0..amt {
            tmp.push(stacks[start as usize - 1].pop_front().unwrap());
        }
        for _ in 0..amt {
            stacks[end as usize - 1].push_front(tmp.pop().unwrap())
        }
    }

    let mut tops = String::new();
    for stack in &mut stacks {
        tops.push(stack.pop_front().unwrap())
    }

    println!("The top crates for the cratemover 9001 are {}.", tops);
}
