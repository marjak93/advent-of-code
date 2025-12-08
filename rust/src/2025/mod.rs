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
        _ => None,
    }
}