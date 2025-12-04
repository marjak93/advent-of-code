const parseArgs = (args: string[]): Map<string, string> => {
  const argMap = new Map<string, string>([["--year", "2025"]]);

  for (const arg of args) {
    const [key, value] = arg.split("=");

    if (!key || !value) {
      console.error(`Invalid argument: ${arg}. Expected format key=value.`);
      Deno.exit(1);
    }

    if (!["--day", "--year", "--part"].includes(key)) {
      console.error(`Unknown argument: ${key}.`);
      Deno.exit(1);
    }

    if (key === "--day") {
      const dayNum = Number(value);
      if (isNaN(dayNum) || dayNum < 1 || dayNum > 12) {
        console.error(`Invalid day value: ${value}. Must be between 1 and 25.`);
        Deno.exit(1);
      }
    }

    if (key === "--year") {
      const yearNum = Number(value);

      if (isNaN(yearNum) || yearNum !== 2025) {
        console.error(
          `Invalid year value: ${value}. Must be between 2025 and 2100.`
        );
        Deno.exit(1);
      }
    }

    argMap.set(key, value);
  }

  return argMap;
};

if (import.meta.main) {
  const args = parseArgs(Deno.args);

  const day = args.get("--day");
  const year = args.get("--year")!;

  if (!day) {
    console.error("Please provide a day number as the first argument.");
    Deno.exit(1);
  }

  // Fetch input from adventofcode.com and create input.txt file
  const session = Deno.env.get("SESSION_ID");

  if (!session) {
    console.error(
      "Please set the SESSION_ID environment variable with your Advent of Code session cookie."
    );
    Deno.exit(1);
  }

  const inputUrl = `https://adventofcode.com/${year}/day/${day}/input`;

  const response = await fetch(inputUrl, {
    headers: {
      Cookie: `session=${session}`,
    },
  });

  if (!response.ok) {
    console.error(
      `Failed to fetch input for day ${day}. Status: ${response.status}`
    );
    Deno.exit(1);
  }

  const inputText = await response.text();

  // Create directory for the day
  const dir = `src/${year}/${day}`;
  await Deno.mkdir(dir, { recursive: true });

  //  Write input to input.txt file
  const inputFilePath = `${dir}/input.txt`;
  await Deno.writeTextFile(inputFilePath, inputText);

  // Create main.ts file with boilerplate code
  const mainTsContent = `import { getInput } from "../../util.ts";

export async function part1() {
  const input = await getInput();
  console.log("Part 1 not implemented yet.");
}

export async function part2() {
  const input = await getInput();
  console.log("Part 2 not implemented yet.");
}
`;
  await Deno.writeTextFile(`${dir}/main.ts`, mainTsContent);
  console.log(`Setup completed for day ${day} in directory ${dir}`);
}
