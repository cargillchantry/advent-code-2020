use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;

pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String> where P: AsRef<Path> {
    let file = File::open(filename).expect("Failed to read file.");
    io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Failed reading line."))
        .filter(|line| !line.is_empty())
}

pub fn read_lines_as_u32<P>(filename: P) -> impl Iterator<Item = u32> where P: AsRef<Path> {
    read_lines(filename)
        .map(|line| u32::from_str(line.as_str()).expect("Failed to convert to u32."))
}
