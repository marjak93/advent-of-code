const parseArgs = (args: string[]): Map<string, string> => {
  const argMap = new Map<string, string>([
    ["--year", "2025"],
    ["--part", "1"],
  ]);

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

    if (key === "--part") {
      const partNum = Number(value);
      if (isNaN(partNum) || (partNum !== 1 && partNum !== 2)) {
        console.error(`Invalid part value: ${value}. Must be 1 or 2.`);
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
  const part = args.get("--part")!;

  if (!day) {
    console.error("Please provide a day number as the first argument.");
    Deno.exit(1);
  }

  // Run the main.ts file for the specified day and year
  const dayModulePath = `./${year}/${day}/main.ts`;
  const dayModule = await import(dayModulePath);

  if (!dayModule) {
    console.error(
      `Could not find module for day ${day} of year ${year} at ${dayModulePath}.`
    );
    Deno.exit(1);
  }

  // Execute the main function if it exists
  const fn = dayModule[`part${part}`];
  if (typeof fn !== "function") {
    console.error(
      `No part${part} function found for day ${day} of year ${year} in ${dayModulePath}.`
    );
    Deno.exit(1);
  }

  await fn();
}
