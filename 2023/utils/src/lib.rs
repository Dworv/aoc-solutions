use std::{fs::File, io::Read};

pub fn rts(day: u32) -> String {
    let mut buf = String::new();
    File::open(format!("inputs/{}.txt", day))
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();
    buf
}