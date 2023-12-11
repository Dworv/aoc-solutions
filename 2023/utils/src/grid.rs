use std::{fs::File, io::{BufReader, BufRead}, fmt::Debug};

pub fn rtg(day: u32) -> CharGrid {
    rtgd(day, ' ')
}

pub fn rtgd(day: u32, default_char: char) -> CharGrid {
    let mut grid = CharGrid {
        rowlen: 0,
        data: Vec::new(),
        default_char
    };
    let mut lines = BufReader::new(File::open(format!("inputs/{}.txt", day)).unwrap()).lines();
    let first = lines.next().unwrap().unwrap();
    grid.rowlen = first.len();
    grid.data.extend(first.chars());
    for line in lines {
        let line = line.unwrap();
        assert_eq!(line.len(), grid.rowlen);
        grid.data.extend(line.chars());
    }
    grid
}

/// X is horizontal, Y is vertical
pub struct CharGrid {
    rowlen: usize,
    data: Vec<char>,
    default_char: char,
}

impl CharGrid {
    pub fn new(size: (usize, usize), fill_char: char, default_char: char) -> Self {
        let mut data = vec![fill_char; size.0 * size.1];
        Self {
            rowlen: size.0,
            data,
            default_char
        }
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) {
        self.data[y * self.rowlen + x] = c;
    }

    pub fn num_rows(&self) -> usize {
        self.data.len() / self.rowlen
    }

    pub fn num_cols(&self) -> usize {
        self.rowlen
    }

    pub fn row(&self, y: usize) -> Row {
        Row {
            grid: self,
            start_x: 0,
            end_x: self.rowlen - 1,
            y
        }
    }

    pub fn col(&self, x: usize) -> Col {
        Col {
            grid: self,
            x,
            start_y: 0,
            end_y: self.data.len() / self.rowlen - 1
        }
    }
}

pub trait GetDefault {
    fn get(&self, x: isize, y: isize) -> char;
}

pub trait GetOption {
    fn get(&self, x: usize, y: usize) -> Option<char>;
}

impl GetDefault for &CharGrid {
    fn get(&self, x: isize, y: isize) -> char {
        if x < 0 || y < 0 {
            return self.default_char;
        }
        self.data.get(y as usize * self.rowlen + x as usize).copied().unwrap_or(self.default_char)
    }
}

impl GetOption for &CharGrid {
    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.data.get(y * self.rowlen + x).copied()
    }
}

pub struct Row<'a> {
    grid: &'a CharGrid,
    start_x: usize,
    end_x: usize,
    y: usize
}

impl<'a> Iterator for Row<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_x > self.end_x {
            return None;
        }
        let res = GetOption::get(&self.grid, self.start_x, self.y);
        self.start_x += 1;
        res
    }
}


impl DoubleEndedIterator for Row<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start_x > self.end_x {
            return None;
        }
        let res = GetOption::get(&self.grid, self.end_x, self.y);
        self.end_x -= 1;
        res
    }
}

pub struct Col<'a> {
    grid: &'a CharGrid,
    x: usize,
    start_y: usize,
    end_y: usize
}

impl<'a> Iterator for Col<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_y > self.end_y {
            return None;
        }
        let res = GetOption::get(&self.grid, self.x, self.start_y);
        self.start_y += 1;
        res
    }
}

impl DoubleEndedIterator for Col<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start_y > self.end_y {
            return None;
        }
        let res = GetOption::get(&self.grid, self.x, self.end_y);
        self.end_y -= 1;
        res
    }
}

impl Debug for CharGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.num_rows() {
            for x in 0..self.num_cols() {
                write!(f, "{}", self.data[y * self.num_cols() + x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
