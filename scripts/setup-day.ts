if (import.meta.main) {
  // Get day from first command line argument
  const year = 2025;
  const day = Deno.args[0];

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
  const dayDir = `src/${day}`;
  await Deno.mkdir(dayDir, { recursive: true });

  //  Write input to input.txt file
  const inputFilePath = `src/${day}/input.txt`;
  await Deno.mkdir(`src/${day}`, { recursive: true });
  await Deno.writeTextFile(inputFilePath, inputText);

  // Create main.ts file with boilerplate code
  const mainTsContent = `if (import.meta.main) {
  const input = await Deno.readTextFile(new URL("input.txt", import.meta.url));

  // Your code here
}
`;
  await Deno.writeTextFile(`${dayDir}/main.ts`, mainTsContent);

  console.log(`Setup completed for day ${day} in directory ${dayDir}`);
}
