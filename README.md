# Pacont
>  A tool to easily gather code context to give to an AI like ChatGPT or Claude.

____

![Release](https://img.shields.io/github/v/release/Tmpecho/pacont)
![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/Rust-1.60+-blue.svg)
![Platform](https://img.shields.io/badge/platform-macOS%20|%20Linux-lightgrey)

## Overview

**Pacont** (short for Path and Content) is a simple and efficient CLI tool designed to recursively print file paths and their contents within specified directories or files. This tool is particularly useful for gathering code context to provide to AI models like ChatGPT or Claude, making it easier to analyze and understand large codebases.

## Features

- Recursively traverse directories and subdirectories.
- Print file paths relative to the input directory.
- Output the contents of each file.
- Accept multiple directories and files as input, concatenating their contents with a separator.

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
pacont <path1> <path2> <path3> ...
```

### Example

Given a directory structure:
```
src/
├── div
│   └── test.kt
└── main.kt
README.md
misc/
├── notes.txt
```

Running `pacont src README.md misc/` will output:
```
**div/test.kt**
//comment

**main.kt**
println("hello")

--------------------------------------------------------------------------------
**README.md**
# Pacont
A very simple CLI tool to recursively print file paths and contents in a directory.

--------------------------------------------------------------------------------
**notes.txt**
Some notes here...
```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request if you have any improvements or new features to add.
