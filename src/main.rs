use std::fs;
use std::io;
use std::{env, path::Path};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprint!("Usage: {} <directory_path>", args[0]);
    }
    let file_path = Path::new(&args[1]);
    read_files_from_dir(file_path)
}

fn read_files_from_dir(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                println!("File: {:?}", path);
            }
        }
    }
    Ok(())
}
