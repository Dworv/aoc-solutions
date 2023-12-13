use utils::rts;

fn main() {
    let input = rts(12);

    part_1(&input);
}

fn part_1(input: &String) {
    let input = parse(input);

    let (springs, records) = &input[0];
    dbg!(arrangements(springs, records, 0, 0, 0));
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

fn arrangements(springs: &Vec<Spring>, records: &Vec<i64>, i: usize, mut record_num: usize, mut damaged_streak: i64) -> i64 {
    if i == springs.len() {
        let valid = (record_num == records.len() - 1 && damaged_streak == records.last().unwrap().clone()) || (record_num == records.len() && damaged_streak == 0);
        return valid as i64;
    }
    match springs[i] {
        Spring::Operational => {
            if damaged_streak > 1 {
                if records[record_num] != damaged_streak {
                    return 0;
                }
                record_num += 1;
            }
            damaged_streak = 0;
            arrangements(springs, records, i + 1, record_num, damaged_streak)
        },
        Spring::Damaged => {
            damaged_streak += 1;
            if damaged_streak > records[record_num] {
                return 0;
            }
            arrangements(springs, records, i + 1, record_num, damaged_streak)
        },
        Spring::Unknown => {
            // operational
            let exiting_streak = damaged_streak > 1;
            let op_arrangements = if !(exiting_streak && records[record_num] != damaged_streak) {
                arrangements(springs, records, i + 1, record_num + exiting_streak as usize, 0)
            } else { 0 };

            // damaged
            let dmg_arrangements = if !(damaged_streak > records[record_num]) {
                arrangements(springs, records, i, record_num, damaged_streak + 1)
            } else { 0 };

            op_arrangements + dmg_arrangements
        },
    }
}

#[derive(Debug, PartialEq, Eq)]
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