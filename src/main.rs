use walkdir::WalkDir;
use std::fs;
use std::env;
use std::str;

fn main() {
    // Use the provided path or default to the current directory.
    let path = env::args().nth(1).unwrap_or_else(|| String::from("."));

    // Traverse the directory recursively using WalkDir.
    for entry in WalkDir::new(&path) {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                // Only attempt to read if the entry is a file.
                if path.is_file() {
                    println!("File: {}", path.display());
                    // Read the file as raw bytes.
                    match fs::read(path) {
                        Ok(bytes) => {
                            // Attempt to convert bytes to a UTF-8 string.
                            match str::from_utf8(&bytes) {
                                Ok(contents) => {
                                    println!("Contents:\n{}", contents);
                                }
                                Err(_) => {
                                    println!("File is not valid UTF-8. Skipping...");
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Could not read {}: {}", path.display(), e);
                        }
                    }
                    println!("---------------------------------------");
                }
            }
            Err(e) => eprintln!("Error reading entry: {}", e),
        }
    }
}
