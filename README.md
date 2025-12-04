# 🎄 Advent of Code

My solutions for [Advent of Code](https://adventofcode.com) implemented in TypeScript using Deno.

## 📋 Prerequisites

- [Deno](https://deno.land/) installed on your system

## 🚀 Quick Start

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

## 📁 Project Structure

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

## 🔧 Configuration

To automatically fetch puzzle inputs, create a `.env` file in the project root:

```env
AOC_SESSION=your_session_cookie_here
```

You can find your session cookie in your browser's developer tools after logging into [adventofcode.com](https://adventofcode.com).

## 📜 License

This project is open source and available under the MIT License.

## 🔗 Links

- [Advent of Code](https://adventofcode.com)
- [Deno Documentation](https://deno.land/manual)
