use utils::rts;

fn main() {
    let input = rts(6);
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let input = parse(input);

    let mut total = 1;
    for (limit, record) in input {
        let limit = limit as f64;
        let record = record as f64;
        let min = (limit-(limit.powf(2.)-4.*record).sqrt())/2.;
        let max = (limit+(limit.powf(2.)-4.*record).sqrt())/2.;
        let min_hold_time = (min + 1.).floor() as u32;
        let max_hold_time = (max - 1.).ceil() as u32;
        total *= max_hold_time - min_hold_time + 1;
    }

    println!("Result: {}", total);
}

fn part_2(input: &str) {
    let (limit, record) = parse_p2(input);
    let limit = limit as f64;
    let record = record as f64;
    let min = (limit-(limit.powf(2.)-4.*record).sqrt())/2.;
    let max = (limit+(limit.powf(2.)-4.*record).sqrt())/2.;
    let min_hold_time = (min + 1.).floor() as u64;
    let max_hold_time = (max - 1.).ceil() as u64;
    println!("Result: {}", max_hold_time - min_hold_time + 1);
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<_>>();

    times.into_iter().zip(distances.into_iter()).collect()
}

fn parse_p2(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    (time, distance)
}