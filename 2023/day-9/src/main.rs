use utils::rts;

fn main() {
    let input = rts(9);

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let mut sum = 0;

    for line in input.lines() {
        let nums: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        sum += predict_next(&nums);
    }

    println!("Result: {}", sum);
}

fn part_2(input: &str) {
    let mut sum = 0;

    for line in input.lines() {
        let nums: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        sum += predict_previous(&nums);
    }

    println!("Result: {}", sum);
}

fn predict_next(nums: &Vec<i64>) -> i64 {
    let mut diffs = vec![];
    let mut all_zero = true;
    for i in 0..(nums.len() - 1) {
        let diff = nums[i + 1] - nums[i];
        if diff != 0 {
            all_zero = false;
        }
        diffs.push(diff);
    }
    if all_zero {
        return nums[0];
    }
    nums.last().unwrap() + predict_next(&diffs)
}

fn predict_previous(nums: &Vec<i64>) -> i64 {
    let mut diffs = vec![];
    let mut all_zero = true;
    for i in 0..(nums.len() - 1) {
        let diff = nums[i + 1] - nums[i];
        if diff != 0 {
            all_zero = false;
        }
        diffs.push(diff);
    }
    if all_zero {
        return nums[0];
    }
    nums[0] - predict_previous(&diffs)
}