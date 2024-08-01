# Blazingly Fast Line Counter

A blazingly fast line counting tool written in Rust, which utilizes parallel processing to count lines in files within a directory. The line counts are categorized by file extension and displayed in a formatted table.

## Features

- **Recursive Directory Traversal**: Reads files from the specified directory and all its subdirectories.
- **Parallel Processing**: Uses the Rayon library to speed up file processing by running tasks in parallel.
- **Formatted Output**: Displays the results in a neatly formatted table, centered in the terminal.

## Requirements

- Rust
- Cargo
- Terminal supporting ANSI escape codes

## Installation

1. **Clone the Repository**:

    ```bash
    git clone https://github.com/mvstermind/li-nes.git
    cd li-nes
    ```

2. **Build the Project**:

    ```bash
    cargo build --release
    ```

## Usage

Run the program from the command line, passing the path to the directory you want to analyze as an argument.

```bash
./target/release/li-nes <directory_path>
```

### Example

```bash
./target/release/li-nes /path/to/your/directory
```

## Output

The program will display the line counts for each file extension found in the specified directory, along with the total number of lines, in a formatted table. The output is centered based on the terminal width if the terminal size can be determined.

### Sample Output

```
                       BLAZINGLY FAST LINE COUNTER
                       ╔═══════════════════════════╗
                       ║ rs            ║      1200 ║
                       ║ txt           ║       800 ║
                       ║ md            ║       450 ║
                       ║ TOTAL         ║      2450 ║
                       ╚═══════════════════════════╝
This took: 0.0023s
```

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any enhancements or bug fixes.
