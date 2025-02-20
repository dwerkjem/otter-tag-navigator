# Otter Tag Navigator

A bespoke AI file tagging system optimized for performance and minimal CPU/GPU overhead on any 64-bit system. Otter Tag Navigator integrates a recursive file browser with semantic tokenization using a prebuilt Hugging Face tokenizer (BERT-base-uncased) to serve as the foundation for an automated file tagging solution.

## Overview

Otter Tag Navigator is a lightweight command-line application written in Rust. It traverses directories recursively, processes files that contain valid UTF-8 content, and tokenizes their text using a state-of-the-art pretrained model. This project is designed to operate efficiently on Debian Linux and other 64-bit systems, making it an ideal starting point for further AI tagging development.

## Features

- **Recursive File Browsing:** Scans directories and subdirectories to locate files.
- **UTF-8 Validation:** Skips files that do not conform to UTF-8 encoding.
- **Semantic Tokenization:** Utilizes the Hugging Face `tokenizers` crate with the BERT-base-uncased model to tokenize file contents.
- **Optimized Performance:** Designed to run with minimal resource overhead, suitable for performance-critical environments.
- **Extensible Foundation:** Provides a solid base for further development toward automated file tagging and AI-based categorization.

## Prerequisites

- **Operating System:** 64-bit Debian Linux (or any 64-bit system)
- **Rust Toolchain:** Install Rust and Cargo via [rustup](https://rustup.rs/)
- **Dependencies:**  
  - `walkdir` for recursive file system traversal  
  - `tokenizers` with the `"remote"` feature for prebuilt tokenizer support

## Installation

1. **Clone the Repository:**

   ```bash
   git clone <repository_url>
   cd otter-tag-navigator
   ```

2. **Build the Project:**

   ```bash
   cargo build
   ```

## Usage

To run the application, specify the target directory. If no directory is provided, the current directory is used by default.

```bash
cargo run -- <directory_path>
```

**Example:**

```bash
cargo run -- testdir
```

### What the Application Does

- **Directory Traversal:** Recursively scans the provided directory for files.
- **UTF-8 Verification:** Reads each file and validates that its content is valid UTF-8. Files failing this check are skipped.
- **Tokenization:** For valid files, the content is tokenized using the BERT-base-uncased tokenizer. The tokens (including special tokens like `[CLS]` and `[SEP]`) are printed to standard output.
- **Error Handling:** The application logs errors encountered during file reading or tokenization, ensuring robust execution.

## Future Development

Potential enhancements include:

- **Tag Prediction:** Incorporate a lightweight classification model to automatically generate semantic tags.
- **Graphical User Interface (GUI):** Develop a GUI to improve usability and file management.
- **Advanced NLP Integration:** Explore additional natural language processing techniques to enhance context-aware tagging.
- **Caching and Performance Tuning:** Implement caching mechanisms to further reduce runtime overhead and improve efficiency.

## Contributing

Contributions are welcome. Please follow these steps:

1. Fork the repository.
2. Create a feature branch.
3. Commit your changes with clear, descriptive messages.
4. Submit a pull request for review.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.