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
    println!("ranges sum: {}", ranges.iter().map(|x| x.end - x.start).sum::<i64>());
    for mut map in maps {
        map.sort_by_key(|(_, start, _)| *start);
        ranges = ranges.iter().flat_map(|range| {
            let mut unmapped = vec![range.clone()];
            let mut mapped = vec![];
            for (dest, source, len) in map.iter() {
                let catch_range = *source..(*source + len);
                let offset = dest - source;
                let mut new_unmapped = vec![];
                for og in unmapped {
                    if catch_range.start <= og.start && og.end <= catch_range.end {
                        mapped.push((og.start + offset)..(og.end + offset));
                    }
                    else if og.start < catch_range.end && catch_range.start <= og.start {
                        mapped.push((og.start + offset)..(catch_range.end + offset));
                        new_unmapped.push(catch_range.end..range.end);
                    }
                    else if range.start < catch_range.start && catch_range.end < range.end {
                        new_unmapped.push(range.start..catch_range.start);
                        mapped.push((catch_range.start + offset)..(catch_range.end + offset));
                        new_unmapped.push(catch_range.end..range.end);
                    }
                    else if catch_range.start < range.end && catch_range.end >= range.end {
                        new_unmapped.push(range.start..catch_range.start);
                        mapped.push((catch_range.start + offset)..(range.end +  offset));
                    } else {
                        new_unmapped.push(og);
                    }
                }
                unmapped = new_unmapped;
            }
            
            mapped.extend(unmapped);
            mapped

        }).collect::<Vec<_>>();

        println!("ranges sum: {}", ranges.iter().map(|x| x.end - x.start).sum::<i64>());
    }
    println!("ranges: {:?}", ranges);
    println!("min: {:?}", ranges.iter().min_by_key(|x| x.start).unwrap().start);
}

#[derive(Debug)]
struct Input {
    seeds: Vec<i64>,
    maps: Vec<Vec<(i64, i64, i64)>>
}

impl Input {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let seeds = lines
            .nth(0)
            .unwrap()
            .split(": ").nth(1)
            .unwrap().split(' ')
            .map(|x| x.parse::<i64>().unwrap())
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
                    parts.next().unwrap().parse::<i64>().unwrap(),
                    parts.next().unwrap().parse::<i64>().unwrap(),
                    parts.next().unwrap().parse::<i64>().unwrap(),
                ));
            }
            maps.push(map);
        }
        Self { seeds, maps }
    }
}