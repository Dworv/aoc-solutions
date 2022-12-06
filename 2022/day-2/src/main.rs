use std::{fs::File, io::{BufReader, prelude::*}, cmp::{Ordering, PartialOrd}};

#[derive(Clone, Copy, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Move::*;
        use Ordering::*;
        Some(match *self {
            Rock => match *other {
                Rock => Equal,
                Paper => Less,
                Scissors => Greater
            }
            Paper => match *other {
                Rock => Greater,
                Paper => Equal,
                Scissors => Less,
            }
            Scissors => match *other {
                Rock => Less,
                Paper => Greater,
                Scissors => Equal
            }
        })
    }
}

impl Move {
    fn from_abc(m: char) -> Move {
        match m {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => panic!("not abc")
        }
    }
    fn from_xyz(m: char) -> Move {
        match m {
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
            _ => panic!("not xyz")
        }
    }
    fn score(self, them: Move) -> u32 {
        let mut s = match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        };
        if self > them {
            s += 6;
        } else if self == them {
            s += 3;
        }
        s
    }
    fn of_outcome(outcome: Ordering, them: Move) -> Move {
        use Ordering::*;
        use Move::*;
        match outcome {
            Greater => match them {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock
            }
            Equal => them,
            Less => match them {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper
            }
        }
    }
}

fn xyz_to_order(xyz: char) -> Ordering {
    use Ordering::*;
    match xyz {
        'X' => Less,
        'Y' => Equal,
        'Z' => Greater,
        _ => panic!("not xyz")
    }
}

fn main() {
    let file = File::open("input.txt").expect("no file");
    let reader = BufReader::new(file);
    
    let mut score = 0;
    for line in reader.lines().map(|x| x.unwrap()) {
        let your_move = Move::from_xyz(line.chars().nth(2).unwrap());
        let their_move = Move::from_abc(line.chars().nth(0).unwrap());
        score += your_move.score(their_move);
    }
    
    println!("Your score is {}.", score);
    
    let file = File::open("input.txt").expect("no file");
    let reader = BufReader::new(file);
    
    let mut score = 0;
    for line in reader.lines().map(|x| x.unwrap()) {
        let their_move = Move::from_abc(line.chars().nth(0).unwrap());
        let outcome = xyz_to_order(line.chars().nth(2).unwrap());
        let your_move = Move::of_outcome(outcome, their_move);
        score += your_move.score(their_move);
    }
    
    println!("If xyz is the outcome, your score is {}.", score);
}