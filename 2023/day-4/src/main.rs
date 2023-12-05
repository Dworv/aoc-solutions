use utils::rts;

fn main() {
    let input = rts(4);
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let (winners, mine) = parse_line(line);
        let score = num_matches(winners, mine);
        if score > 0 {
            sum += 2u32.pow(score - 1);
        }
    }
    println!("Result: {}", sum);
}

fn part_2(input: &str) {
    let mut match_list = vec![];
    for line in input.lines() {
        let (winners, mine) = parse_line(line);
        let score = num_matches(winners, mine);
        match_list.push(score);
    }

    let mut copy_list = vec![1; match_list.len()];

    for (i, num_matches) in match_list.into_iter().enumerate() {
        for j in dbg!((i+1)..=(i+num_matches as usize)) {
            copy_list[j] += copy_list[i];
        }
    }

    println!("Result: {}", copy_list.iter().sum::<u32>());
}

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let mut parts = line.split_once(':').unwrap().1.split('|').map(|x| {
        x.trim()
            .split(' ')
            .filter_map(|x| x.trim().parse::<u32>().ok())
            .collect::<Vec<u32>>()
    });
    let winners = parts.next().unwrap();
    let mine = parts.next().unwrap();
    (winners, mine)
}

fn num_matches(mut a: Vec<u32>, b: Vec<u32>) -> u32 {
    a.sort();
    let mut num_wins = 0;
    for num in b {
        if a.binary_search(&num).is_ok() {
            num_wins += 1;
        }
    }
    num_wins
}
