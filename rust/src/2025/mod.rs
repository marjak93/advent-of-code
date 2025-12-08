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
        _ => None,
    }
}