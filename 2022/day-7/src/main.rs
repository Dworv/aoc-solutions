use std::{fs::File, io::{BufRead, BufReader, Read, Lines}};

trait LinesExt<T: BufRead> {
    fn with_peek(self) -> PeekableLines<T>;
}

impl<T: BufRead> LinesExt<T> for Lines<T> {
    fn with_peek(self) -> PeekableLines<T> {
        PeekableLines { inner: self, saved: None }
    }
}

struct PeekableLines<T: BufRead> {
    inner: Lines<T>,
    saved: Option<String>
}

impl<T: BufRead> Iterator for PeekableLines<T> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(s) = self.saved {
            self.saved = None;
            Some(s)
        } else { self.inner.next().map(|x| x.unwrap()) }
    }
}

impl<T: BufRead> PeekableLines<T> {
    fn peek(&mut self) -> Option<String> {
        let n = self.next();
        self.saved = n;
        n
    }
}

fn search(lines: &mut PeekableLines<BufReader<File>>, candidates: &mut Vec<u32>) -> u32 {
    let mut size = 0;
    loop {
        if let Some(line) = lines.next() {
            if !line.starts_with('d') {
                size += line.split(" ").next().unwrap().parse::<u32>().unwrap();
            }
            if lines.peek().unwrap_or_default().starts_with("$") {
                break
            }
        } else {
            
            break 
        }
    }
    loop {
        if let Some(line) = lines.next() {

            if lines.peek().unwrap_or_default().starts_with("$") {
                break
            }
        } else { break }
    }
    candidates.push(size);
    size
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().with_peek();
    let mut candidates = vec![];
    search(&mut lines, &mut candidates);

    println!("Hello, world!");
}
