use crate::util::get_input;

fn calculate_joltage(input: String, n: usize) -> u64 {
    input
        .lines()
        .map(|bank| {
            let batteries: Vec<u64> = bank
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u64))
                .collect();

            let mut result = 0u64;
            let mut start_idx = 0;

            for digits_remaining in (1..=n).rev() {
                // Find the rightmost position with the maximum digit
                // that leaves enough batteries for remaining digits
                let search_end = batteries.len() - digits_remaining + 1;

                let (best_idx, best_digit) = (start_idx..search_end)
                    .rev() // Scan from right to left
                    .map(|i| (i, batteries[i]))
                    .max_by_key(|&(_, digit)| digit)
                    .unwrap();

                result = result * 10 + best_digit;
                start_idx = best_idx + 1;
            }

            result
        })
        .sum()
}

#[allow(unused_variables)]
pub fn part1() {
    let input = get_input(2025, 3);

    let joltage = calculate_joltage(input, 2);

    println!("Part 1 result: {}", joltage);
}

#[allow(unused_variables)]
pub fn part2() {
    let input = get_input(2025, 3);

    let joltage = calculate_joltage(input, 12);

    println!("Part 2 result: {}", joltage);
}
