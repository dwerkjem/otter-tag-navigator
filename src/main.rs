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
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    println!("File: {}", path.display());
                    // Read file contents as raw bytes.
                    match fs::read(path) {
                        Ok(bytes) => {
                            // Attempt to convert bytes to a UTF-8 string.
                            match str::from_utf8(&bytes) {
                                Ok(contents) => {
                                    println!("Contents:\n{}", contents);
                                    // Encode the contents using the prebuilt tokenizer.
                                    let encoding = tokenizer.encode(contents, true)
                                        .expect("Tokenization failed.");
                                    println!("Tokens:");
                                    // Iterate over and print each token.
                                    for token in encoding.get_tokens() {
                                        println!("{}", token);
                                    }
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
