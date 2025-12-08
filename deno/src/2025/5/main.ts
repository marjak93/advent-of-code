import { getInput } from "../../util.ts";

export async function part1() {
  const input = await getInput(import.meta.url);

  const [freshRangesStr, availableIngredientsStr] = input.split("\n\n");

  const freshRanges = freshRangesStr
    .split("\n")
    .map((r) => r.split("-").map((n) => +n));

  const availableIngredients = availableIngredientsStr
    .split("\n")
    .map((n) => +n);

  let n = 0;

  for (const ingredient of availableIngredients) {
    for (const [start, end] of freshRanges) {
      if (ingredient >= start && ingredient <= end) {
        n++;
        break;
      }
    }
  }

  console.log(n);
}

export async function part2() {
  const input = await getInput(import.meta.url);

  const [_ranges] = input.split("\n\n");

  const ranges = _ranges
    .split("\n")
    .map((r) => r.split("-").map((n) => +n))
    .toSorted((a, b) => a[0] - b[0]) as [number, number][];

  const mergedRanges: [number, number][] = [];

  for (let i = 0; i < ranges.length; i++) {
    if (mergedRanges.length === 0) {
      mergedRanges.push(ranges[i]);
      continue;
    }

    const last = mergedRanges[mergedRanges.length - 1];
    const currentRange = ranges[i];

    if (currentRange[0] <= last[1] + 1) {
      last[1] = Math.max(last[1], currentRange[1]);
    } else {
      mergedRanges.push(currentRange);
    }
  }

  let n = 0;

  for (const [start, end] of mergedRanges) {
    n += end - start + 1;
  }

  console.log(n);
}
