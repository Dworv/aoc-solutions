use std::collections::HashMap;

use utils::rts;

fn main() {
    let input = rts(8);
    
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let (instructions, map) = parse(input);

    let mut loc = Node::new("AAA");
    let mut inst_loops = 0;

    while loc != Node::new("ZZZ") {
        for instruction in &instructions {
            let (left, right) = map.get(&loc).unwrap();
            let next = match instruction {
                Direction::Left => left,
                Direction::Right => right
            };
            loc = next.clone();
        }
        inst_loops += 1;
    }

    println!("Result: {}", inst_loops * instructions.len());
}

fn part_2(input: &str) {
    let (instructions, map) = parse(input);

    let mut locs = vec![];

    for key in map.keys() {
        if key.code[2] == 'A' as u8 {
            locs.push(key.clone());
        }
    }

    let mut loc_loops = vec![];

    for mut loc in locs {
        let mut inst_loops = 0;

        while loc.code[2] != 'Z' as u8 {
            for instruction in &instructions {
                let (left, right) = map.get(&loc).unwrap();
                let next = match instruction {
                    Direction::Left => left,
                    Direction::Right => right
                };
                loc = next.clone();
            }
            inst_loops += 1;
        }

        loc_loops.push(inst_loops);
    }

    println!("Result: {}", loc_loops.iter().fold(1, |acc, x| lcm(acc, *x)) * instructions.len() as u64);
}

fn parse(input: &str) -> (Vec<Direction>, HashMap<Node, (Node, Node)>) {
    let mut lines = input.lines();

    let mut instructions = vec![];
    for c in lines.next().unwrap().chars() {
        instructions.push(match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction")
        });
    }
    lines.next(); // skip empty line

    let mut map = HashMap::new();

    for line in lines {
        let start = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        map.insert(Node::new(start), (Node::new(left), Node::new(right)));
    }

    (instructions, map)
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Node {
    code: [u8; 3]
}

impl Node {
    fn new(s: &str) -> Self {
        let s = s.chars().map(|c| c as u8).collect::<Vec<_>>();
        Self { code: [s[0], s[1], s[2]] }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.code[0] as char, self.code[1] as char, self.code[2]as char)
    }
}

enum Direction {
    Left,
    Right
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcf(a, b)
}

fn gcf(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcf(b, a % b)
}
