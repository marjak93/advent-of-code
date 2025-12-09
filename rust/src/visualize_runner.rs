use clap::Parser;

#[path = "util.rs"]
mod util;
#[path = "2025/mod.rs"]
mod year2025;

#[derive(Parser, Debug)]
#[command(name = "visualize")]
#[command(about = "Run visualization for a specific day", long_about = None)]
struct Args {
    #[arg(long)]
    day: u32,

    #[arg(long)]
    part: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match (args.day, args.part) {
        (9, 2) => {
            // Call day 9 part 2 visualizer
            year2025::day9::visualize::run_visualizer().await;
        }
        _ => {
            eprintln!(
                "No visualizer available for day {} part {}",
                args.day, args.part
            );
            std::process::exit(1);
        }
    }
}
