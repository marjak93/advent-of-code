const checkValidity = (n: number): boolean => {
  const str = n.toString();

  // Figure out number of factors of the length of the string. E.g, if length is 6, factors are 1, 2, 3, 6
  // However, avoid 6 itself, as that would mean checking the whole number against itself.
  const length = str.length;
  const factors: number[] = [];

  for (let i = 1; i <= Math.floor(length / 2); i++) {
    if (length % i === 0) {
      factors.push(i);
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

if (import.meta.main) {
  const input = await Deno.readTextFile(new URL("input.txt", import.meta.url));
  const ranges = input.split(",");

  let sum = 0;

  for (const range of ranges) {
    const [start, end] = range.split("-").map((n) => +n);

    for (let i = start; i <= end; i++) {
      const isValid = checkValidity(i);

      if (!isValid) {
        sum += i;
      }
    }
  }

  console.log(sum);
}
