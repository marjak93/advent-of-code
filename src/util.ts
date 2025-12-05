export function getInput(dir: string) {
  return Deno.readTextFile(new URL("input.txt", dir));
}

export function getDebug(dir: string) {
  return Deno.readTextFile(new URL("debug.txt", dir));
}
