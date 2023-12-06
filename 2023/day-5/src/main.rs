use utils::rts;

fn main() {
    let input = rts(5);
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let Input { seeds: mut items, maps } = Input::parse(input);
    for map in maps {
        for item in &mut items {
            if let Some((dest, source, _)) = map.iter().find(|(_, start, len)| {
                (*start..(start + len)).contains(item)
            }) {
                *item = dest + (*item - source);
            }
        }
    }
    println!("Results: {}", items.iter().min().unwrap());
}

fn part_2(input: &str) {
    let Input { seeds: items, maps } = Input::parse(input);
    let mut ranges = items.chunks(2).map(|x| x[0]..(x[0] + x[1])).collect::<Vec<_>>();
    println!("ranges sum: {}", ranges.iter().map(|x| x.end - x.start).sum::<u64>());
    for mut map in maps {
        map.sort_by_key(|(_, start, _)| *start);
        ranges = ranges.iter().flat_map(|range| {
            let mut new_ranges = vec![range.clone()];

            for (dest, source, len) in map.iter() {
                let starts = new_ranges.clone();
                let catch_range = *source..(*source + len);
                let dest_range = *dest..(*dest + len);
                if catch_range.start <= range.start && catch_range.end >= range.end {
                    new_ranges[0] = (dest_range.start + (range.start - catch_range.start))..(dest_range.start + (range.end - catch_range.start));
                }
                else if catch_range.end > range.start && catch_range.start <= range.start {
                    new_ranges[0] = (dest_range.end - (catch_range.end - range.start))..dest_range.end;
                    new_ranges.push(catch_range.end..range.end);
                }
                else if catch_range.start >= range.start && catch_range.end <= range.end {
                    if catch_range.start > new_ranges.last().unwrap().start {
                        new_ranges.last_mut().unwrap().end = catch_range.start;
                    }
                    new_ranges.push(dest_range.clone());
                    if range.end > catch_range.end {
                        new_ranges.push(catch_range.end..range.end);
                    }
                }
                else if catch_range.start < range.end && catch_range.end >= range.end {
                    new_ranges.last_mut().unwrap().end = catch_range.start;
                    new_ranges.push(dest_range.start..(dest_range.start + (range.end - catch_range.start)));
                }
                let mut copy = new_ranges.clone();
                copy.sort_by_key(|x| x.start);
                for i in 0..(copy.len() - 1) {
                    if copy[i].end > copy.get(i+1).unwrap_or(&(100000000..100000000)).start {
                        println!("FAUILURE FOUND \n MAP: {:?} {:?} -> {:?} \n STARTS: {:?} \n ENDS: {:?}\n", (dest, source, len), catch_range, dest_range, starts, new_ranges);
                    }
                }
            }

            new_ranges

        }).collect::<Vec<_>>();

        println!("ranges sum: {}", ranges.iter().map(|x| x.end - x.start).sum::<u64>());
    }
    println!("ranges: {:?}", ranges);
    println!("min: {:?}", ranges.iter().min_by_key(|x| x.start).unwrap().start);
}

#[derive(Debug)]
struct Input {
    seeds: Vec<u64>,
    maps: Vec<Vec<(u64, u64, u64)>>
}

impl Input {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let seeds = lines
            .nth(0)
            .unwrap()
            .split(": ").nth(1)
            .unwrap().split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let mut maps = vec![];
        lines.next();
        'maps: loop { // all maps
            lines.next(); // skip 2 lines
            let mut map = vec![];
            loop { // map ranges
                let line = if let Some(line) = lines.next() {
                    line
                } else {
                    maps.push(map);
                    break 'maps;
                };
                if line.is_empty() {
                    break;
                }
                let mut parts = line.split(' ');
                map.push((
                    parts.next().unwrap().parse::<u64>().unwrap(),
                    parts.next().unwrap().parse::<u64>().unwrap(),
                    parts.next().unwrap().parse::<u64>().unwrap(),
                ));
            }
            maps.push(map);
        }
        Self { seeds, maps }
    }
}