use walkdir::WalkDir;
use std::env;
use std::fs;
use std::str;
use tokenizers::Tokenizer;

fn main() {
    // Use the provided path or default to the current directory.
    let path = env::args().nth(1).unwrap_or_else(|| String::from("."));

    // Load a prebuilt tokenizer. This example uses the BERT base uncased tokenizer.
    // Note: This will attempt to download tokenizer files on the first run.
    let tokenizer = Tokenizer::from_file("tokenizer.json")
        .expect("Failed to load tokenizer from file.");

    // Traverse the directory recursively using WalkDir.
    for entry in WalkDir::new(&path) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error reading entry: {}", e);
                continue;
            }
        };

        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        println!("File: {}", path.display());

        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Could not read {}: {}", path.display(), e);
                continue;
            }
        };

        let contents = match str::from_utf8(&bytes) {
            Ok(content) => content,
            Err(_) => {
                println!("File is not valid UTF-8. Skipping...");
                continue;
            }
        };

        println!("Contents:\n{}", contents);

        let encoding = match tokenizer.encode(contents, true) {
            Ok(encoding) => encoding,
            Err(e) => {
                eprintln!("Tokenization failed: {}", e);
                continue;
            }
        };

        println!("Tokens:");
        for token in encoding.get_tokens() {
            println!("{}", token);
        }
        println!("---------------------------------------");
    }
}
