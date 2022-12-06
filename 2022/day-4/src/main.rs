use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;
    for line in reader.lines().map(|x| x.unwrap()) {
        let raws = line.split(",");
        let mut ranges: [(u32, u32); 2] = Default::default();
        for (i, raw) in raws.take(2).enumerate() {
            let mut nums = raw.split("-");
            ranges[i].0 = nums.next().unwrap().parse().unwrap();
            ranges[i].1 = nums.next().unwrap().parse().unwrap();
        }

        if (ranges[0].0 <= ranges[1].0 && ranges[0].1 >= ranges[1].1)
            || (ranges[1].0 <= ranges[0].0 && ranges[1].1 >= ranges[0].1) {
                total += 1;
            }
    }
    println!("The total number of fully overlapping ranges is {}", total);

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;
    for line in reader.lines().map(|x| x.unwrap()) {
        let raws = line.split(",");
        let mut ranges: [(u32, u32); 2] = Default::default();
        for (i, raw) in raws.take(2).enumerate() {
            let mut nums = raw.split("-");
            ranges[i].0 = nums.next().unwrap().parse().unwrap();
            ranges[i].1 = nums.next().unwrap().parse().unwrap();
        }

        if ranges[0].1 >= ranges[1].0 || ranges[0].0 <= ranges[1].1 {
            total += 1;
        }
    }
    println!("The total number of at-all overlapping ranges is {}", total);
}
