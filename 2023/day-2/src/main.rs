use utils::rts;

fn main() {
    let input = rts(2);
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let mut sum = 0;
    for (mut game_num, line) in input.lines().enumerate() {
        game_num += 1; // start at 1

        let colors = parse_line(line);
        if colors.iter().all(|c| c.is_possible()) {
            sum += game_num;
        }
    }

    println!("{}", sum);
}

fn part_2(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let colors = parse_line(line);
        let min_amt = colors.into_iter().fold(Colors { r: 0, g: 0, b: 0 }, |acc, c| {
            Colors {
                r: acc.r.max(c.r),
                g: acc.g.max(c.g),
                b: acc.b.max(c.b),
            }
        });
        let power = min_amt.r * min_amt.g * min_amt.b;
        sum += power;
    }

    println!("{}", sum);

}

fn parse_line(line: &str) -> Vec<Colors> {
    let handfuls = line.split(':').nth(1).unwrap().trim().split(';');
    let mut res = Vec::new();
    for handful in handfuls {
        let amts = handful.split(',');
        let mut colors = Colors { r: 0, g: 0, b: 0 };
        for mut amt in amts {
            amt = amt.trim();
            let (count, color) = amt.split_once(' ').unwrap();
            match color {
                "red" => colors.r = count.parse().unwrap(),
                "green" => colors.g = count.parse().unwrap(),
                "blue" => colors.b = count.parse().unwrap(),
                _ => panic!("Unknown color: {}", color),
            }
        }
        res.push(colors);
    }
    res
}

#[derive(Debug)]
struct Colors {
    r: u32,
    g: u32,
    b: u32,
}

impl Colors {
    fn is_possible(&self) -> bool {
        self.r <= 12 && self.g <= 13 && self.b <= 14
    }
}