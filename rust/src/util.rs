use std::fs;
use std::path::PathBuf;

pub fn get_input(year: u32, day: u32) -> String {
    let path = PathBuf::from(format!("src/{}/{}/input.txt", year, day));
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read input file at {:?}", path))
}

pub fn read_input_lines(year: u32, day: u32) -> Vec<String> {
    get_input(year, day)
        .lines()
        .map(|s| s.to_string())
        .collect()
}
