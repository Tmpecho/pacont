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