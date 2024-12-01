mod solutions;
mod data;

use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    day: u32,

    #[arg(short, long, default_value_t = false)]
    all: bool,
}

const SOLUTIONS_PATH: &str = "src/solutions/";

fn main() {
    println!("\n|---------------------------|");
    println!("|    Advent of Code 2024    |");
    println!("|---------------------------|\n");
    let args = Args::parse();

    if !args.all && args.day == 0 {
        println!("Nothing to run here.");
        return;
    }

    let mut day_map: HashMap<u8, fn()> = HashMap::new();
    day_map.insert(1, solutions::day1::main);

    let solution_file = format!("{}day{}.rs", SOLUTIONS_PATH, args.day);
    if !args.all{
        if !Path::new(solution_file.as_str()).exists(){
            println!("Trying to run a non existent solution, {}" ,solution_file);
            return;
        }

        run_function_day(&mut day_map, solution_file);
        return;
    }

    println!("Running all solutions \n");
    match fs::read_dir(SOLUTIONS_PATH) {
        Ok(all_files) => {
            for file in all_files {
                match file {
                    Ok(entry) => {
                        if entry.path().is_file() && !entry.path().display().to_string().contains("mod"){
                            run_function_day(&mut day_map, entry.path().display().to_string());
                        }
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error opening directory: {}", e),
    }
}

fn run_function_day(day_map: &mut HashMap<u8, fn()>, solution_file: String) {
    let day = extract_day_number(solution_file.as_str());

    if let Some(day_function) = day_map.get(&day) {
        day_function(); // Call the selected function
    } else {
        eprintln!("No solution found for '{}'.", day);
    }
}

fn extract_day_number(path: &str) -> u8 {
    // Define a regex pattern that matches the number after "day" and before ".rs"
    let re = Regex::new(r"day(\d+)\.rs").unwrap();

    re.captures(path)
        .and_then(|n| n[1].parse::<u8>().ok()) // Parse it as u8
        .unwrap_or(0) // If parsing fails, return 0 (or any other default value)
}