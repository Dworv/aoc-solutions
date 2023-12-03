use utils::rtc;

fn main() {
    let input = rtc(1);
    sum_lines(&input, false);
    sum_lines(&input, true);
}

fn sum_lines(input: &Vec<char>, replace: bool) {
    let res = input.split(|c| *c == '\n').fold(0, |acc, line| {
        let mut nums = (0, 0);
        let mut first_found = false;

        if replace {
            let line = replace_words(line);
        }

        for c in line {
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

fn replace_words(input: &[char]) -> Vec<char> {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];
    let mut res = Vec::new();
    
}