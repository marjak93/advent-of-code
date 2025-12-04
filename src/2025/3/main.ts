if (import.meta.main) {
  const input = await Deno.readTextFile(new URL("input.txt", import.meta.url));

  const banks = input.split("\n");

  const results: number[] = [];

  for (const bank of banks) {
    const batteries = bank.split("").map((j) => +j);

    const digits: number[] = [];

    // Part 1 -- n = 2
    // Part 2 -- n = 12
    const n = 12;

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

  console.log(results.reduce((a, b) => a + b, 0));
}
