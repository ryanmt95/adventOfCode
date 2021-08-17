use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn read_numbers(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
}