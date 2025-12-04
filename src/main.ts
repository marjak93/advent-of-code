if (import.meta.main) {
  const day = Deno.args[0];
  const year = 2025;

  if (!day) {
    console.error("Please provide a day number as the first argument.");
    Deno.exit(1);
  }
}
