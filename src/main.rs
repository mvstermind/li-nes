use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use term_size;

/// Main function to handle command-line input and execute the line counting logic.
fn main() -> Result<(), io::Error> {
    // Record the start time for performance measurement
    let time_start = Instant::now();

    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Check if the user provided a directory path argument
    if args.len() < 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        return Ok(());
    }

    // Get the directory path from the command-line arguments
    let dir_path = Path::new(&args[1]);

    // Read files from the directory and get line counts by file extension
    let line_counts = read_files_from_dir(dir_path)?;

    // Calculate the total number of lines across all file extensions
    let total_lines: usize = line_counts.values().sum();

    // Prepare output lines for displaying results in a table format
    let mut output = Vec::new();
    output.push("\n".to_string());
    output.push("BLAZINGLY FAST LINE COUNTER".to_string());
    output.push("╔═══════════════════════════╗".to_string());
    for (ext, count) in &line_counts {
        // Format each line of the table with file extension and line count
        output.push(format!("║ {:<13} ║ {:>9} ║", ext, count));
    }
    // Add the total line count to the table
    output.push(format!("║ {:<13} ║ {:>9} ║", "TOTAL", total_lines));
    output.push("╚═══════════════════════════╝".to_string());

    // Find the length of the longest line for centering the output
    let max_line_length = output.iter().map(|line| line.len()).max().unwrap_or(0);

    // Get the terminal width to center the output text
    if let Some((width, _)) = term_size::dimensions() {
        for line in output {
            // Calculate padding for centering the output
            let padding = (width.saturating_sub(max_line_length)) / 2;
            println!("{:padding$}{}", "", line, padding = padding);
        }
    } else {
        // If terminal size can't be determined, print without centering
        for line in output {
            println!("{}", line);
        }
    }

    // Record the end time and calculate the duration
    let time_end = Instant::now();
    let time_diff = time_end.duration_since(time_start);
    println!("This took: {:?}", time_diff);
    Ok(())
}

/// Recursively reads files from the directory and counts lines by file extension.
/// Utilizes parallel processing to speed up the directory traversal.
fn read_files_from_dir(dir: &Path) -> io::Result<HashMap<String, usize>> {
    // Create a thread-safe shared HashMap to store line counts
    let line_counts = Arc::new(Mutex::new(HashMap::new()));

    if dir.is_dir() {
        // Collect directory entries into a vector for parallel processing
        let entries: Vec<_> = fs::read_dir(dir)?.collect();

        // Process directory entries in parallel
        entries.into_par_iter().for_each(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();

            // If the entry is a file, count lines based on its extension
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    let ext = ext.to_string();
                    let count = count_lines_in_file(&path).unwrap_or(0);
                    let mut line_counts = line_counts.lock().unwrap();
                    *line_counts.entry(ext).or_insert(0) += count;
                }
            } else if path.is_dir() {
                // If the entry is a directory, recursively process it
                let sub_dir_counts = read_files_from_dir(&path).unwrap_or_default();
                let mut line_counts = line_counts.lock().unwrap();
                for (ext, count) in sub_dir_counts {
                    *line_counts.entry(ext).or_insert(0) += count;
                }
            }
        });
    }

    // Unwrap the Arc and Mutex to return the final HashMap
    Ok(Arc::try_unwrap(line_counts).unwrap().into_inner().unwrap())
}

/// Counts the number of lines in a file using a buffered reader.
/// Returns the line count as an integer.
fn count_lines_in_file(file_path: &Path) -> io::Result<usize> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let count = reader.lines().count();
    Ok(count)
}
