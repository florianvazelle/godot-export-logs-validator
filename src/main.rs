use clap::Parser;
use colored::*;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

/// A simple tool to check Godot export logs for issues.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Godot export log file
    #[arg()]
    log_file: String,

    /// Fail the program if any warnings are found
    #[arg(short, long)]
    fail_on_warnings: bool,
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Use the log file path from the arguments
    let log_file_path = args.log_file;

    // Attempt to open the log file
    if let Ok(lines) = read_lines(log_file_path) {
        let mut has_errors = false;

        println!("Checking Godot export log for issues...");

        // Process each line in the file
        for line in lines.map_while(Result::ok) {
            let line = remove_ansi_escape_codes(&line);
            has_errors |= lint(&line, args.fail_on_warnings);
        }

        // If issues were found, exit with a non-zero status code
        if has_errors {
            println!("One or more issues were found in the log file.");
            process::exit(1); // Exit with error code 1
        } else {
            println!("No issues found in the log file.");
            process::exit(0); // Exit successfully with code 0
        }
    } else {
        eprintln!("Failed to read the log file. Please check the file path.");
        process::exit(1); // Exit with error code 1 if the file can't be read
    }
}

/// Reads lines from a file and returns an iterator
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Checks if a line contains any errors or warnings based on user flags
fn lint(line: &str, fail_on_warnings: bool) -> bool {
    if line.contains("ERROR") {
        println!("{}", line.red());
        true
    } else if line.contains("WARNING") && fail_on_warnings {
        println!("{}", line.yellow());
        true
    } else {
        false
    }
}

/// Removes ANSI escape codes from the log string
fn remove_ansi_escape_codes(log: &str) -> String {
    // Regex to match ANSI escape codes (color codes)
    let ansi_regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();

    // Replace escape codes with an empty string to remove them
    ansi_regex.replace_all(log, "").to_string()
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    // Test case for lint function
    #[test]
    fn test_lint() {
        assert!(!lint("Missing: res://textures/missing_texture.png", false));
        assert!(lint(
            "ERROR: Failed to load script res://scripts/nonexistent.gd",
            false
        ));
        assert!(!lint("Export started...", false));
        assert!(!lint("Export completed.", false));
    }

    // Test case for read_lines function
    #[test]
    fn test_read_lines_valid_file() {
        // Create a temporary file for testing
        let temp_file_path = "test_log.txt";
        let content = "Missing: res://textures/missing_texture.png\nERROR: Failed to load script res://scripts/nonexistent.gd\nExport completed.";

        std::fs::write(temp_file_path, content).expect("Unable to write to file");

        let lines = read_lines(temp_file_path).expect("Failed to read lines");
        let lines: Vec<String> = lines.filter_map(Result::ok).collect();

        // Check that the file contents were read correctly
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "Missing: res://textures/missing_texture.png");
        assert_eq!(
            lines[1],
            "ERROR: Failed to load script res://scripts/nonexistent.gd"
        );
        assert_eq!(lines[2], "Export completed.");

        // Clean up the temporary file
        std::fs::remove_file(temp_file_path).expect("Unable to delete file");
    }

    // Test case for read_lines with invalid file path
    #[test]
    fn test_read_lines_invalid_file() {
        let lines = read_lines("non_existent_log.txt");
        assert!(lines.is_err()); // Should return an error for non-existent file
    }
}
