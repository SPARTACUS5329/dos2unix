# DOS to UNIX File Converter

This is a simple Rust project that converts DOS-style line endings (`\r\n`) to Unix-style line endings (`\n`). It reads a file, processes it, and then writes the contents back to the same file with the correct line endings.

## Features

- Recursively reads files in a given directory.
- Converts all DOS line endings (`\r\n`) to Unix line endings (`\n`).
- Supports reading and writing to files in a safe and efficient manner.

## Installation

### Prerequisites

- You must have [Rust](https://www.rust-lang.org/) installed on your machine.
- If you don't have Rust installed, you can install it by following the instructions at [rust-lang.org](https://www.rust-lang.org/learn/get-started).

### Running the Project

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/dos-to-unix-converter.git
   cd dos-to-unix-converter
   ```
  
2. Run the project
   ```bash
   cargo run <path-to-your-file>
   cargo run <accepts-regex>
   ```
