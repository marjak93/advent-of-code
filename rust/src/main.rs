pub mod util;

#[path = "2025/mod.rs"]
pub mod year2025;

use clap::Parser;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day number (1-25)
    #[arg(short, long)]
    day: u32,

    /// Year
    #[arg(short, long, default_value_t = 2025)]
    year: u32,

    /// Part number (1 or 2)
    #[arg(short, long, default_value_t = 1)]
    part: u32,
}

fn main() {
    let args = Args::parse();

    if args.day < 1 || args.day > 25 {
        eprintln!("Invalid day value: {}. Must be between 1 and 25.", args.day);
        std::process::exit(1);
    }

    if args.part != 1 && args.part != 2 {
        eprintln!("Invalid part value: {}. Must be 1 or 2.", args.part);
        std::process::exit(1);
    }

    if args.year != 2025 {
        eprintln!("Invalid year value: {}. Only 2025 is supported.", args.year);
        std::process::exit(1);
    }

    let start = Instant::now();

    let solution = match args.year {
        2025 => year2025::get_solution(args.day, args.part),
        _ => None,
    };

    match solution {
        Some(func) => func(),
        None => {
            eprintln!(
                "Solution not implemented for year {}, day {}, part {}",
                args.year, args.day, args.part
            );
            std::process::exit(1);
        }
    }

    let duration = start.elapsed();
    println!("\nExecution Time: {:?}", duration);
}
