use utils::rts;

fn main() {
    let input = rts(7);
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let input = parse(input, false);
    let mut scores = input.iter().map(|(hand, bid)| (hand.score(), *bid)).collect::<Vec<_>>();
    scores.sort_by_key(|x| x.0);

    let sum = scores.iter().enumerate().fold(0, |acc, (i, (_, bid))| acc + (i as u32 + 1) * *bid);

    println!("Result: {}", sum);
}

fn part_2(input: &str) {
    let input = parse(input, true);
    let mut scores = input.iter().map(|(hand, bid)| (hand.score_wildcard(), *bid)).collect::<Vec<_>>();
    scores.sort_by_key(|x| x.0);

    let sum = scores.iter().enumerate().fold(0, |acc, (i, (_, bid))| acc + (i as u32 + 1) * *bid);

    println!("Result: {}", sum);
}

fn parse(input: &str, weak_j: bool) -> Vec<(Hand, u32)> {
    input.lines().map(|x| x.split_once(' ').unwrap()).map(|(hand, bid)| (Hand::from_str(hand, weak_j), bid.parse::<u32>().unwrap())).collect()
}

#[derive(Debug)]
struct Hand {
    cards: [u8; 5]
}

impl Hand {
    fn from_str(input: &str, weak_j: bool) -> Self {
        let mut cards = [0; 5];
        for card in input.chars().enumerate() {
            cards[card.0] = match card.1 {
                'T' => if weak_j { 10 } else { 9 },
                'J' => if !weak_j { 10 } else { 0 },
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                _ => card.1.to_digit(10).unwrap() as u8 - !weak_j as u8
            };
        }
        Self { cards }
    }

    fn score(&self) -> u32 {
        // get combos
        let mut freqs: Vec<(u8, i32)> = vec![];
        for card in self.cards.iter() {
            if let Some(freq) = freqs.iter_mut().find(|x| x.0 == *card) {
                freq.1 += 1;
            } else {
                freqs.push((*card, 1));
            }
        }

        let mut freqs: Vec<i32> = freqs.into_iter().map(|(_, freq)| freq).collect();
        freqs.sort_by_key(|x| -x);

        let base = match freqs.as_slice() {
            [5] => 8,
            [4, 1] => 7,
            [3, 2] => 6,
            [3, 1, 1] => 3,
            [2, 2, 1] => 2,
            [2, 1, 1, 1] => 1,
            [1, 1, 1, 1, 1] => 0,
            _ => panic!("Invalid hand: {:?}, {:?}", self, freqs)
        };

        let extra = self.cards.iter().enumerate().fold(0, |acc, (i, card)|
            acc + 14u32.pow(4 - i as u32) * *card as u32
        );

        base * 14u32.pow(5) + extra
    }

    fn score_wildcard(&self) -> u32 {
        // get combos
        let mut num_wildcards = 0;
        let mut freqs: Vec<(u8, i32)> = vec![];
        for card in self.cards.iter() {
            if *card == 0 {
                num_wildcards += 1;
                continue;
            } else if let Some(freq) = freqs.iter_mut().find(|x| x.0 == *card) {
                freq.1 += 1;
            } else {
                freqs.push((*card, 1));
            }
        }

        let mut freqs: Vec<i32> = freqs.into_iter().map(|(_, freq)| freq).collect();
        freqs.sort_by_key(|x| -x);
        if num_wildcards == 5 {
            freqs.push(5);
        } else {
            freqs[0] += num_wildcards;
        }

        let base = match freqs.as_slice() {
            [5] => 8,
            [4, 1] => 7,
            [3, 2] => 6,
            [3, 1, 1] => 3,
            [2, 2, 1] => 2,
            [2, 1, 1, 1] => 1,
            [1, 1, 1, 1, 1] => 0,
            _ => panic!("Invalid hand: {:?}, {:?}", self, freqs)
        };

        println!("hand: {:?}, base: {}", self, base);

        let extra = self.cards.iter().enumerate().fold(0, |acc, (i, card)|
            acc + 14u32.pow(4 - i as u32) * *card as u32
        );

        base * 14u32.pow(5) + extra
    }
}
