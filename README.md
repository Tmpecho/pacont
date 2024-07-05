# Pacont

![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/Rust-1.60+-blue.svg)

## Overview

**Pacont** (short for Path and Content) is a simple and efficient CLI tool designed to recursively print file paths and their contents within a specified directory. This tool is particularly useful for gathering code context to provide to AI models like ChatGPT or Claude, making it easier to analyze and understand large codebases.

## Features

- Recursively traverse directories and subdirectories.
- Print file paths relative to the input directory.
- Output the contents of each file.

## Installation

### Prerequisites

- Rust and Cargo installed. If you haven't installed Rust yet, you can do so by running:

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

### Build from Source

1.	Clone the repository:
```bash
git clone https://github.com/Tmpecho/pacont.git
cd pacont
```

2.	Build the project:
```bash
cargo build --release
```

3.	Move the executable to a directory in your PATH:

#### MacOS/Linux:
```bash
sudo cp target/release/pacont /usr/local/bin/pacont
```

### Homebrew Installation
```bash
brew tap Tmpecho/pacont
brew install pacont
```

## Usage
```bash
pacont <directory>
```

### Example

Given a directory structure:
```
src/
├── div
│   └── test.kt
└── main.kt
```

Running `pacont src` will output:
```
**div/test.kt**
// Code here

**main.kt**
// Code here
```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request if you have any improvements or new features to add.
