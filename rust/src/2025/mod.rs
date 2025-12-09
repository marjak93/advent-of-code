#[path = "9/mod.rs"]
pub mod day9;
#[path = "6/mod.rs"]
pub mod day6;
#[path = "5/mod.rs"]
pub mod day5;
#[path = "4/mod.rs"]
pub mod day4;
#[path = "2/mod.rs"]
pub mod day2;
#[path = "3/mod.rs"]
pub mod day3;
#[path = "7/mod.rs"]
pub mod day7;
#[path = "1/mod.rs"]
pub mod day1;
#[path = "8/mod.rs"]
pub mod day8;

pub fn get_solution(day: u32, part: u32) -> Option<fn()> {
    match (day, part) {
        (8, 1) => Some(day8::part1),
        (8, 2) => Some(day8::part2),
        (1, 1) => Some(day1::part1),
        (1, 2) => Some(day1::part2),
        (7, 1) => Some(day7::part1),
        (7, 2) => Some(day7::part2),
        (3, 1) => Some(day3::part1),
        (3, 2) => Some(day3::part2),
        (2, 1) => Some(day2::part1),
        (2, 2) => Some(day2::part2),
        (4, 1) => Some(day4::part1),
        (4, 2) => Some(day4::part2),
        (5, 1) => Some(day5::part1),
        (5, 2) => Some(day5::part2),
        (6, 1) => Some(day6::part1),
        (6, 2) => Some(day6::part2),
        (9, 1) => Some(day9::part1),
        (9, 2) => Some(day9::part2),
        _ => None,
    }
}