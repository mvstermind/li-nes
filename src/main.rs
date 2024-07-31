use std::collections::HashMap; // For storing line counts by file extension
use std::fs; // For file system operations
use std::io::{self, BufRead}; // For reading files and handling input/output
use std::time::Instant;
use std::{env, path::Path}; // For handling command-line arguments and paths
use term_size; // For getting terminal dimensions to center output

fn main() -> Result<(), io::Error> {
    // Collect command-line arguments into a vector
    let time_start = Instant::now();

    let args: Vec<String> = env::args().collect();

    // Check if the directory path argument is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        return Ok(());
    }

    // Get the directory path from the command-line arguments
    let dir_path = Path::new(&args[1]);

    // Read files from the specified directory and count lines by file extension
    let line_counts = read_files_from_dir(dir_path)?;

    // Calculate total lines
    let total_lines: usize = line_counts.values().sum();

    // Prepare the output lines for the table
    let mut output = Vec::new();
    output.push("\n".to_string());
    output.push("BLAZINGLY FAST LINE COUNTER".to_string());
    output.push("╔═════════════════════════╗".to_string());
    for (ext, count) in &line_counts {
        // Format each line of the table with file extension and line count
        output.push(format!("║ {:<13} ║ {:>7} ║", ext, count));
    }
    // Add the total line count
    output.push(format!("║ {:<13} ║ {:>7} ║", "TOTAL", total_lines));
    output.push("╚═════════════════════════╝".to_string());

    // Find the length of the longest line for centering
    let max_line_length = output.iter().map(|line| line.len()).max().unwrap_or(0);

    // Get the terminal width to center the output
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

    let time_end = Instant::now();
    let time_diff = time_end.duration_since(time_start);
    println!("This took: {:?}", time_diff);
    Ok(())
}

// Function to read files from the directory and count lines by extension
fn read_files_from_dir(dir: &Path) -> io::Result<HashMap<String, usize>> {
    let mut line_counts = HashMap::new(); // Create a HashMap to store line counts

    if dir.is_dir() {
        // Iterate over each entry in the directory
        for entry in fs::read_dir(dir)? {
            let entry = entry?; // Get the directory entry
            let path = entry.path(); // Get the path of the entry

            if path.is_file() {
                // If the entry is a file, process it
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    let ext = ext.to_string(); // Get the file extension as a string
                    let count = count_lines_in_file(&path)?; // Count lines in the file
                    *line_counts.entry(ext).or_insert(0) += count; // Update line count for the extension
                }
            } else if path.is_dir() {
                // If the entry is a directory, recursively process it
                let sub_dir_counts = read_files_from_dir(&path)?;
                for (ext, count) in sub_dir_counts {
                    *line_counts.entry(ext).or_insert(0) += count; // Update line counts from subdirectories
                }
            }
        }
    }

    Ok(line_counts)
}

// Function to count the number of lines in a file
fn count_lines_in_file(file_path: &Path) -> io::Result<usize> {
    let file = fs::File::open(file_path)?; // Open the file
    let reader = io::BufReader::new(file); // Create a buffered reader
    let count = reader.lines().count(); // Count the number of lines in the file
    Ok(count)
}

