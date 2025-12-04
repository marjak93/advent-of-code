const checkValidity = (n: number, mode: "simple" | "advanced"): boolean => {
  const str = n.toString();
  const length = str.length;

  const factors: Set<number> = new Set([length / 2]);

  if (mode === "advanced") {
    // Figure out number of factors of the length of the string. E.g, if length is 6, factors are 1, 2, 3, 6
    // However, avoid 6 itself, as that would mean checking the whole number against itself.
    for (let i = 1; i <= Math.floor(length / 2); i++) {
      if (length % i === 0) {
        factors.add(i);
      }
    }
  }

  //   console.log(`Number: ${n}, Factors: ${factors}`);

  // For each factor, split the string into parts of that length and check if all parts are the same.
  for (const factor of factors) {
    const parts: string[] = [];

    for (let i = 0; i < length; i += factor) {
      parts.push(str.slice(i, i + factor));
    }

    // console.log(`  Factor: ${factor}, Parts: ${parts}`);

    const allSame = parts.every((part) => part === parts[0]);

    if (allSame) {
      //   console.log(`    All same: ${allSame}`);
      return false;
    }
  }

  return true;
};

export async function part1() {
  const input = await getInput();
  const ranges = input.split(",");

  let sum = 0;

  for (const range of ranges) {
    const [start, end] = range.split("-").map((n) => +n);

    for (let i = start; i <= end; i++) {
      const isValid = checkValidity(i, "simple");

      if (!isValid) {
        sum += i;
      }
    }
  }

  console.log(sum);
}

export async function part2() {
  const input = await getInput();
  const ranges = input.split(",");

  let sum = 0;

  for (const range of ranges) {
    const [start, end] = range.split("-").map((n) => +n);

    for (let i = start; i <= end; i++) {
      const isValid = checkValidity(i, "advanced");

      if (!isValid) {
        sum += i;
      }
    }
  }

  console.log(sum);
}

function getInput() {
  return Deno.readTextFile(new URL("input.txt", import.meta.url));
}
