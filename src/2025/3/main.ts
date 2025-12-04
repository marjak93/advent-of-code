import { getInput } from "../../util.ts";

const calculateJoltage = (input: string, n: number) => {
  const banks = input.split("\n");

  const results: number[] = [];

  for (const bank of banks) {
    const batteries = bank.split("").map((j) => +j);

    const digits: number[] = [];

    // Look for the occurance of 9 in the bank that has at least n - i digits after it.
    // If not found, look for 8, then 7, etc.
    while (digits.length < n) {
      let target = 9;

      let found = false;

      while (!found && target >= 0) {
        for (let i = 0; i < batteries.length; i++) {
          if (
            batteries[i] === target &&
            batteries.length - i >= n - digits.length
          ) {
            digits.push(batteries[i]);
            batteries.splice(0, i + 1);
            found = true;
            break;
          }
        }

        target--;
      }
    }

    results.push(+digits.join(""));
  }

  return results.reduce((a, b) => a + b, 0);
};

export async function part1() {
  const input = await getInput(import.meta.url);

  const joltage = calculateJoltage(input, 2);

  console.log(joltage);
}

export async function part2() {
  const input = await getInput(import.meta.url);

  const joltage = calculateJoltage(input, 12);

  console.log(joltage);
}
