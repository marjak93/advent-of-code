import { getInput } from "../../util.ts";

// Rotate the assymetric matrix 90 degrees clockwise
const rotate90 = <T extends unknown>(_mat: T[][]) => {
  // Deep copy the matrix to avoid mutating the original
  const mat = JSON.parse(JSON.stringify(_mat)) as T[][];

  const rows = mat.length;
  const cols = mat[0].length;
  const rotated: T[][] = [];

  for (let c = 0; c < cols; c++) {
    const newRow: T[] = [];
    for (let r = rows - 1; r >= 0; r--) {
      newRow.push(mat[r][c]);
    }
    rotated.push(newRow);
  }

  return rotated;
};

export async function part1() {
  const input = await getInput(import.meta.url);

  const lines = input.split("\n").filter(Boolean);
  const parts = lines.map((l) => l.split(" ").filter(Boolean));

  const problems = rotate90(parts);

  let result = 0;

  for (const line of problems) {
    const [op, ...args] = line;

    if (op === "+") {
      result += args.reduce((a, b) => a + +b, 0);
    }

    if (op === "*") {
      result += args.reduce((a, b) => a * +b, 1);
    }
  }

  console.log(result);
}

export async function part2() {
  const input = await getInput(import.meta.url);

  const rows = input.split("\n").filter(Boolean);
  const cols = rows.map((r) => r.split(""));

  // Find the maximum length of the columns
  const max = Math.max(...cols.map((c) => c.length));

  // Pad the columns to make a rectangular matrix
  const paddedCols = cols.map((c) => {
    while (c.length < max) {
      c.push(" ");
    }

    return c;
  });

  const rotated = rotate90(paddedCols);

  let i = 0;
  const problems = [];

  // Separate the rotated rows into individual problems
  for (const row of rotated) {
    if (row.every((c) => c === " ")) {
      i++;
      continue;
    }

    if (!problems[i]) {
      problems[i] = [] as string[][];
    }

    problems[i].push(row);
  }

  let total = 0;

  // Solve each problem
  for (let i = 0; i < problems.length; i++) {
    const problem = problems[i];

    let operator;
    let result = 0;

    for (let j = 0; j < problem.length; j++) {
      const l = problem[j];

      let digits: string[] = [];

      if (j === 0) {
        [operator, ...digits] = l;
      } else {
        digits = l;
      }

      const n = +digits.toReversed().join("").trim();

      if (result === 0) {
        result = n;
      } else {
        if (operator === "+") {
          result += n;
        } else if (operator === "*") {
          result *= n;
        }
      }
    }

    total += result;
  }

  console.log(total);
}
