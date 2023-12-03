use std::{fs::File, io::Read};

pub fn rtc(day: u32) -> Vec<char> {
    let mut buf = String::new();
    File::open(format!("inputs/{}.txt", day))
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();
    buf.chars().collect()
}