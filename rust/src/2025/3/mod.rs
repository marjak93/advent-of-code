use crate::util::get_input;

fn calculate_joltage(input: String, n: usize) -> u64 {
    let banks = input.lines();

    let mut results = Vec::new();

    for bank in banks {
        let mut batteries: Vec<u64> = bank
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u64))
            .collect();

        let mut digits: Vec<u64> = Vec::new();

        while digits.len() < n {
            let mut target = 9;
            let mut found = false;

            while !found {
                for i in 0..batteries.len() {
                    if batteries[i] == target && batteries.len() - i >= n - digits.len() {
                        digits.push(batteries[i]);
                        batteries = batteries.split_off(i + 1);
                        found = true;

                        break;
                    }
                }

                target -= 1;
            }
        }

        let result: u64 = digits.iter().fold(0, |acc, &d| acc * 10 + d);
        results.push(result);
    }

    results.iter().sum()
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
