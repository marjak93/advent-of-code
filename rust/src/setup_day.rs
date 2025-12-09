use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Setup a new Advent of Code day", long_about = None)]
struct Args {
    /// Day number (1-25)
    #[arg(short, long)]
    day: u32,

    /// Year
    #[arg(short, long, default_value_t = 2025)]
    year: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let args = Args::parse();

    if args.day < 1 || args.day > 25 {
        eprintln!("Invalid day value: {}. Must be between 1 and 25.", args.day);
        std::process::exit(1);
    }

    if args.year != 2025 {
        eprintln!("Invalid year value: {}. Only 2025 is supported.", args.year);
        std::process::exit(1);
    }

    // Fetch input from adventofcode.com
    let session = std::env::var("AOC_SESSION")
        .expect("AOC_SESSION environment variable not set. Please add it to your .env file.");

    let input_url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        args.year, args.day
    );
    let client = reqwest::Client::new();
    let response = client
        .get(&input_url)
        .header("Cookie", format!("session={}", session))
        .send()
        .await?;

    if !response.status().is_success() {
        eprintln!(
            "Failed to fetch input for day {}. Status: {}",
            args.day,
            response.status()
        );
        std::process::exit(1);
    }

    let input_text = response.text().await?;

    // Create directory structure
    let dir = PathBuf::from(format!("src/{}/{}", args.year, args.day));
    fs::create_dir_all(&dir)?;

    // Write input file
    let input_path = dir.join("input.txt");
    fs::write(&input_path, input_text)?;

    // Create mod.rs with boilerplate
    let mod_content = format!(
        r#"use crate::util::get_input;
use rayon::prelude::*;

#[allow(unused_variables)]
pub fn part1() {{
    let input = get_input({}, {});
    println!("Part 1 not implemented yet.");
}}

#[allow(unused_variables)]
pub fn part2() {{
    let input = get_input({}, {});
    println!("Part 2 not implemented yet.");
}}
"#,
        args.year, args.day, args.year, args.day
    );

    fs::write(dir.join("mod.rs"), mod_content)?;

    println!(
        "Setup completed for day {} in directory {:?}",
        args.day, dir
    );

    // Update year module file
    update_year_module(args.year, args.day)?;

    println!(
        "\n✓ Setup complete! Run with: cargo run --bin aoc -- --day {} --part 1",
        args.day
    );

    Ok(())
}

fn update_year_module(year: u32, day: u32) -> std::io::Result<()> {
    let year_dir = PathBuf::from(format!("src/{}", year));
    let mod_file = year_dir.join("mod.rs");

    // Create year directory if it doesn't exist
    fs::create_dir_all(&year_dir)?;

    // Read existing content or create new
    let content = if mod_file.exists() {
        fs::read_to_string(&mod_file)?
    } else {
        String::new()
    };

    let lines: Vec<&str> = content.lines().collect();
    let mut new_lines = Vec::new();
    let mut found_get_solution = false;
    let mut inserted_module = false;
    let mut inserted_match = false;

    // Check if module declaration exists
    let mod_declaration = format!("#[path = \"{}/mod.rs\"]\npub mod day{};", day, day);
    let simple_check = format!("pub mod day{};", day);
    let has_module = content.contains(&simple_check);

    for line in lines.iter() {
        // Insert module declaration at the top if not present
        if !inserted_module && !has_module && line.starts_with("#[path") {
            new_lines.push(mod_declaration.clone());
            inserted_module = true;
        }

        new_lines.push(line.to_string());

        // Find the get_solution function and add the new day
        if line.contains("pub fn get_solution") {
            found_get_solution = true;
        }

        // Insert before the _ => None line in get_solution
        if found_get_solution && !inserted_match && line.trim().starts_with("_ => None") {
            new_lines.insert(
                new_lines.len() - 1,
                format!("        ({}, 1) => Some(day{}::part1),", day, day),
            );
            new_lines.insert(
                new_lines.len() - 1,
                format!("        ({}, 2) => Some(day{}::part2),", day, day),
            );
            inserted_match = true;
        }
    }

    // If no module declaration was added yet, add it at the beginning
    if !has_module && !inserted_module {
        new_lines.insert(0, mod_declaration);
    }

    // If get_solution function doesn't exist, create it
    if !found_get_solution {
        new_lines.push(String::new());
        new_lines.push("pub fn get_solution(day: u32, part: u32) -> Option<fn()> {".to_string());
        new_lines.push("    match (day, part) {".to_string());
        new_lines.push(format!("        ({}, 1) => Some(day{}::part1),", day, day));
        new_lines.push(format!("        ({}, 2) => Some(day{}::part2),", day, day));
        new_lines.push("        _ => None,".to_string());
        new_lines.push("    }".to_string());
        new_lines.push("}".to_string());
    }

    fs::write(&mod_file, new_lines.join("\n"))?;
    println!("✓ Updated {}/mod.rs with day{}", year, day);

    Ok(())
}
