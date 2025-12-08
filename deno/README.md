# 🎄 Advent of Code 2025

My solutions for [Advent of Code 2025](https://adventofcode.com/2025) implemented in TypeScript (Deno) and Rust.

## 📋 Prerequisites

### TypeScript Version
- [Deno](https://deno.land/) installed on your system

### Rust Version
- [Rust](https://www.rust-lang.org/) (1.70 or later) installed on your system

## 🚀 Quick Start

## TypeScript (Deno)

### Setup a New Day

To scaffold files for a new puzzle day:

```bash
deno task setup-day --day=<day> [--year=2025]
```

Example:
```bash
deno task setup-day --day=5
```

This will create:
- `src/<year>/<day>/main.ts` - Solution template
- `src/<year>/<day>/input.txt` - Input file (fetched automatically from AoC if session token is configured)

### Run a Solution

To run a specific day's solution:

```bash
deno task start --day=<day> [--year=2025] [--part=1]
```

Examples:
```bash
# Run day 1, part 1 (default)
deno task start --day=1

# Run day 3, part 2
deno task start --day=3 --part=2

# Run with specific year
deno task start --day=5 --year=2025 --part=1
```

### Development Mode

Run in watch mode for faster iteration:

```bash
deno task dev --day=<day> [--year=2025] [--part=1]
```

## Rust

### Setup a New Day

Navigate to the `rust/` directory and run:

```bash
cd rust
cargo run --bin setup-day -- --day <day> [--year 2025]
```

Example:
```bash
cargo run --bin setup-day -- --day 5
```

This will create:
- `src/year<year>/day<day>/mod.rs` - Solution template
- `src/year<year>/day<day>/input.txt` - Input file (fetched automatically from AoC if session token is configured)

### Run a Solution

```bash
cd rust
cargo run --bin aoc -- --day <day> [--year 2025] [--part 1]
```

Examples:
```bash
# Run day 1, part 1 (default)
cargo run --bin aoc -- --day 1

# Run day 3, part 2
cargo run --bin aoc -- --day 3 --part 2

# Run in release mode for better performance
cargo run --release --bin aoc -- --day 8 --part 1
```

## 📁 Project Structure

### TypeScript (Deno)

```
advent-of-code/
├── deno.json              # Deno configuration and tasks
├── src/
│   ├── main.ts           # Main runner script
│   ├── setup-day.ts      # Day scaffolding script
│   ├── util.ts           # Shared utilities
│   └── 2025/
│       ├── 1/
│       │   ├── main.ts
│       │   └── input.txt
│       ├── 2/
│       │   ├── main.ts
│       │   └── input.txt
│       └── ...
```

### Rust

```
rust/
├── Cargo.toml            # Rust project configuration
├── src/
│   ├── main.rs          # Main runner
│   ├── setup_day.rs     # Day scaffolding script
│   ├── util.rs          # Shared utilities
│   └── 2025/
│       ├── mod.rs
│       ├── 1/
│       │   ├── mod.rs
│       │   └── input.txt
│       ├── 2/
│       │   ├── mod.rs
│       │   └── input.txt
│       └── ...
```

## 🔧 Configuration

To automatically fetch puzzle inputs, create a `.env` file in the project root (for TypeScript) or in the `rust/` directory (for Rust):

```env
AOC_SESSION=your_session_cookie_here
```

You can find your session cookie in your browser's developer tools after logging into [adventofcode.com](https://adventofcode.com).

## 📝 Solution Templates

### TypeScript (Deno)

```typescript
import { getInput } from "../../util.ts";

const solvePart1 = (input: string): number => {
  // Solution for part 1
  return 0;
};

const solvePart2 = (input: string): number => {
  // Solution for part 2
  return 0;
};

export default async function main(part: number) {
  const input = await getInput(import.meta.url);
  
  if (part === 1) {
    console.log("Part 1:", solvePart1(input));
  } else {
    console.log("Part 2:", solvePart2(input));
  }
}
```

### Rust

```rust
use crate::util::get_input;

pub fn part1() {
    let input = get_input(2025, 1);
    // Solution for part 1
    println!("Part 1 not implemented yet.");
}

pub fn part2() {
    let input = get_input(2025, 1);
    // Solution for part 2
    println!("Part 2 not implemented yet.");
}
```

## ✨ Utilities

### TypeScript: `getInput(dir: string)`

Helper function to read the `input.txt` file from the current day's directory.

```typescript
import { getInput } from "../../util.ts";

const input = await getInput(import.meta.url);
```

### Rust: `get_input(year: u32, day: u32)`

Helper function to read the `input.txt` file for a specific day.

```rust
use crate::util::get_input;

let input = get_input(2025, 1);
```

## 🎯 Progress

- [x] Day 1
- [x] Day 2
- [x] Day 3
- [x] Day 4
- [ ] Day 5
- [ ] ...

## 📜 License

This project is open source and available under the MIT License.

## 🔗 Links

- [Advent of Code 2025](https://adventofcode.com/2025)
- [Deno Documentation](https://deno.land/manual)
- [Rust Documentation](https://doc.rust-lang.org/)
