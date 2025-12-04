export function getInput(dir: string) {
  return Deno.readTextFile(new URL("input.txt", dir));
}
