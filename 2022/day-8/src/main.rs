use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug)]
enum Countable<T> {
    Counted(T),
    Uncounted(T)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut forest: Vec<Vec<Countable<i8>>> = vec![];

    let mut lines = reader.lines().map(|x| x.unwrap());
    for c in lines.next().unwrap().chars() {
        forest.push(vec![Countable::Uncounted(c as i8 - '0' as i8)]) // convert char digit to i8 by subtracting offset
    }

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            forest[i].push(Countable::Uncounted(c as i8 - '0' as i8))// convert char digit to i8 by subtracting offset
        }
    }

    let mut visible = 0u32;

    // colums
    for col in forest.iter_mut() {
        let mut highest = -1;
        let mut stop_at = None;
        for (i, tree) in col.iter_mut().enumerate() {
            match tree {
                Countable::Counted(ct) => highest = highest.max(*ct),
                Countable::Uncounted(ut) => {
                    if *ut > highest {
                        highest = *ut;
                        *tree = Countable::Counted(*ut);
                        visible += 1;
                    }
                },
            }
            if highest == 9 {
                stop_at = Some(i);
                break
            }
        }
        let len = col.len(); // have to do it now to dodge borrow checks :)
        highest = -1;
        for tree in col.iter_mut().rev().take(len - stop_at.unwrap_or(0)) {
            match tree {
                Countable::Counted(ct) => highest = highest.max(*ct),
                Countable::Uncounted(ut) => {
                    if *ut > highest {
                        highest = *ut;
                        *tree = Countable::Counted(*ut);
                        visible += 1;
                    }
                },
            }
            if highest == 9 {
                break
            }
        }
    }

    // rows
    let len = forest[0].len();
    for i in 0..len {
        let mut highest = -1;
        let mut stop_at = None;
        for (j, tree) in forest.iter_mut().map(|x| x.get_mut(i).unwrap()).enumerate() {
            match tree {
                Countable::Counted(ct) => highest = highest.max(*ct),
                Countable::Uncounted(ut) => {
                    if *ut > highest {
                        highest = *ut;
                        *tree = Countable::Counted(*ut);
                        visible += 1;
                    }
                },
            }
            if highest == 9 {
                stop_at = Some(j);
                break
            }
        }
        highest = -1;
        let row_len = forest.len();
        for tree in forest.iter_mut().map(|x| x.get_mut(i).unwrap()).rev().take(row_len - stop_at.unwrap_or(0)) {
            match tree {
                Countable::Counted(ct) => highest = highest.max(*ct),
                Countable::Uncounted(ut) => {
                    println!("row {}, {} > {}", i, ut, highest);
                    if *ut > highest {
                        highest = *ut;
                        *tree = Countable::Counted(*ut);
                        visible += 1;
                    }
                },
            }
            if highest == 9 {
                break
            }
        }
    }
    // dbg!(forest);
    dbg!(visible);
}
