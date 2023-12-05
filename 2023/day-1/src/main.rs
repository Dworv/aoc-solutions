use utils::rts;

fn main() {
    let input = rts(1);
    sum_lines(&input, false);
    sum_lines(&input, true);
}

fn sum_lines(input: &str, replace: bool) {
    let res = input.lines().fold(0, |acc, line| {
        let mut nums = (0, 0);
        let mut first_found = false;

        let line = if replace {
            replace_words(line)
        } else {
            line.to_string()
        };

        for c in line.chars() {
            if c.is_digit(10) {
                if !first_found {
                    first_found = true;
                    let num = c.to_digit(10).unwrap();
                    nums.0 = num;
                    nums.1 = num;
                } else {
                    nums.1 = c.to_digit(10).unwrap();
                }
            }
        }

        acc + nums.0 * 10 + nums.1
    });

    println!("Result: {}", res);
}

fn replace_words(input: &str) -> String {
    let words = [
        ("one", "one1one"), ("two", "two2two"), ("three", "three3three"), ("four", "four4four"), ("five", "five5five"),
        ("six", "six6six"), ("seven", "seven7seven"), ("eight", "eight8eight"), ("nine", "nine9nine"),
    ];
    let mut res = input.to_string();
    for (from, to) in words.iter() {
        res = res.replace(from, to);
    }
    res
}
