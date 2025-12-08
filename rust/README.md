# Rust Implementation - Advent of Code 2025

This directory contains the Rust implementation of the Advent of Code 2025 solutions.

## Building the Project

```bash
cargo build
```

For optimized builds:
```bash
cargo build --release
```

## Running Solutions

```bash
cargo run --bin aoc -- --day <day> [--year 2025] [--part 1]
```

## Setting Up New Days

```bash
cargo run --bin setup-day -- --day <day> [--year 2025]
```

## Configuration

Create a `.env` file in this directory:

```env
AOC_SESSION=your_session_cookie_here
```

## Project Structure

- `src/main.rs` - Main runner that dispatches to specific day solutions
- `src/setup_day.rs` - Script to scaffold new day directories
- `src/util.rs` - Shared utility functions
- `src/2025/` - Solutions organized by year and day (e.g., `src/2025/1/`, `src/2025/2/`)
