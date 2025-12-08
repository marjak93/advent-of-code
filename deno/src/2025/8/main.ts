import { getDebug, getInput } from "../../util.ts";

type JunctionBox = [number, number, number];
type Circuit = [number, ...number[]];

const calculateDistance = (a: JunctionBox, b: JunctionBox): number => {
  const [x1, y1, z1] = a;
  const [x2, y2, z2] = b;

  return Math.sqrt((x2 - x1) ** 2 + (y2 - y1) ** 2 + (z2 - z1) ** 2);
};

const createCircuits = (
  junctionBoxes: Map<number, JunctionBox>,
  maxIterations: number
) => {
  let last: { a: number; b: number } | null = null;
  const distances: { a: number; b: number; distance: number }[] = [];

  for (let i = 0; i < junctionBoxes.size; i++) {
    for (let j = i + 1; j < junctionBoxes.size; j++) {
      const dist = calculateDistance(
        junctionBoxes.get(i)!,
        junctionBoxes.get(j)!
      );
      distances.push({ a: i, b: j, distance: dist });
    }
  }

  distances.sort((a, b) => a.distance - b.distance);

  const circuits: Circuit[] = [];

  for (const [index] of junctionBoxes) {
    circuits.push([index]);
  }

  let iteration = 0;

  while (iteration < maxIterations) {
    const { a, b } = distances.shift()!;

    // Connect circuit with a to circuit with b
    const circuitAIndex = circuits.findIndex((c) => c.includes(a));
    const circuitBIndex = circuits.findIndex((c) => c.includes(b));

    // Already connected
    if (circuitAIndex === circuitBIndex) {
      // console.log(
      //   `\nIteration ${iteration}: Junction box ${a} and ${b} already connected, skipping`
      // );

      iteration++;

      continue;
    }

    // Merge circuits
    circuits[circuitAIndex] = [
      ...circuits[circuitAIndex],
      ...circuits[circuitBIndex],
    ];

    // console.log(
    //   `\nIteration ${iteration}: Connecting junction box ${a} and ${b}`
    // );

    circuits.splice(circuitBIndex, 1);

    // Stop if only one circuit remains
    if (circuits.length === 1) {
      last = { a, b };

      break;
    }

    // console.log(circuits);

    iteration++;
  }

  return { circuits, last };
};

export async function part1() {
  const input = await getInput(import.meta.url);

  const junctionBoxes: Map<number, JunctionBox> = new Map(
    input
      .split("\n")
      .filter(Boolean)
      .map((line, i) => {
        const [x, y, z] = line.split(",").map((d) => +d);
        return [i, [x, y, z] as JunctionBox];
      })
  );

  const { circuits } = createCircuits(junctionBoxes, 1000);

  const lengths = circuits.map((c) => c.length);

  const threeLargest = lengths.sort((a, b) => b - a).slice(0, 3);

  const result = threeLargest.reduce((acc, val) => acc * val, 1);

  console.log(result);
}

export async function part2() {
  const input = await getInput(import.meta.url);

  const junctionBoxes: Map<number, JunctionBox> = new Map(
    input
      .split("\n")
      .filter(Boolean)
      .map((line, i) => {
        const [x, y, z] = line.split(",").map((d) => +d);
        return [i, [x, y, z] as JunctionBox];
      })
  );

  const { last } = createCircuits(junctionBoxes, Infinity);

  if (last) {
    const a = junctionBoxes.get(last.a)!;
    const b = junctionBoxes.get(last.b)!;

    console.log(a[0] * b[0]);
  }
}
