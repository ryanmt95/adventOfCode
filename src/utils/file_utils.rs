use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn read_numbers(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect()
}

pub fn read_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("File does not exist");
    let reader = BufReader::new(file);
    reader.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}