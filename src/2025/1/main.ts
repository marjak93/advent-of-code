if (import.meta.main) {
  const input = await Deno.readTextFile(new URL("input.txt", import.meta.url));

  const instructions = input.split("\n").map((line) => line.trim());

  /*
   * a = start location
   * b = end location
   * x = distance to move
   * k = number of full loops
   * i = total number of times crossed 0
   * */

  let a = 50;
  let b = 50;
  let i = 0;

  for (const instruction of instructions) {
    const [, dir, val] = instruction.match(/([L|R])(\d+)/) ?? [];

    const x = +val;

    a = b;

    let k = Math.floor(x / 100);
    const r = x % 100;

    if (dir === "L") {
      b = (a - r + 100) % 100;

      if (a !== 0 && b > a) {
        k += 1;
      }

      if (b === 0) {
        k++;
      }
    }

    if (dir === "R") {
      b = (a + r) % 100;

      if (b < a) {
        k += 1;
      }
    }

    i += k;
    console.log({ dir, x, a, b, k, i });
  }
}
