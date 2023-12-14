use utils::rts;

use cached::{proc_macro::cached, SizedCache};

fn main() {
    let input = rts(12);

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &String) {
    let input = parse(input);

    let mut sum = 0;
    for (springs, records) in input {
        sum += arrangements(&springs, &records, 0);
    }

    println!("Result: {}", sum);
}

fn part_2(input: &String) {
    let input = parse(input);

    let mut sum = 0;
    for (mut springs, mut records) in input {
        springs.push(Spring::Unknown);
        springs = springs.repeat(5);
        springs.pop();
        records = records.repeat(5);
        let total = arrangements(&springs, &records, 0);
        sum += total;
    }

    println!("Result: {}", sum);
}

fn parse(input: &String) -> Vec<(Vec<Spring>, Vec<i64>)> {
    input.lines().map(|line| {
        let (springs, recs) = line.split_once(' ').unwrap();
        (
            springs.chars().map(Spring::from).collect(),
            recs.split(',').map(|x| x.parse().unwrap()).collect()
        )
    }).collect()
}

#[cached(
    type = "SizedCache<String, i64>",
    create = "{ SizedCache::with_size(10000) }",
    convert = r#"{ format!("{:?}|{:?}|{}", springs, records, damaged_streak) }"#
)]
fn arrangements(springs: &[Spring], records: &[i64], damaged_streak: i64) -> i64 {
    if springs.len() == 0 {
        let valid = (records.len() == 1 && damaged_streak == records.last().unwrap().clone()) || (records.len() == 0 && damaged_streak == 0);

        return valid as i64;
    }
    match springs[0] {
        Spring::Operational => {
            let offset = if damaged_streak >= 1 {
                if *records.get(0).unwrap_or(&0) != damaged_streak {
                    return 0;
                }
                1
            } else { 0 };
            arrangements(&springs[1..], &records[offset..], 0)
        },
        Spring::Damaged => {
            arrangements(&springs[1..], records, damaged_streak + 1)
        },
        Spring::Unknown => {
            // operational
            let exiting_streak = damaged_streak >= 1;
            let op_arrangements = if !(exiting_streak && *records.get(0).unwrap_or(&0) != damaged_streak) {
                arrangements(&springs[1..], &records[exiting_streak as usize..], 0)
            } else { 0 };

            // damaged
            let dmg_arrangements = if !(damaged_streak > *records.get(0).unwrap_or(&0)) {
                arrangements(&springs[1..], records, damaged_streak + 1)
            } else { 0 };

            op_arrangements + dmg_arrangements
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!()
        }
    }
}