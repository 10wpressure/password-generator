# Password Generator

A secure and customizable password generator written in Rust.

## Features
- Customizable password length (1-255 characters)
- Option to include/exclude numbers, letters, and special characters
- Secure random number generation
- Command-line interface

## Installation
```bash
cargo install password-generator
```

## Usage
```bash
password-generator --length 16 --numbers --letters --symbols
```

## Options
- `-l, --length <LENGTH>`: Set password length (default: 24)
- `-n, --numbers`: Include numbers
- `-l, --letters`: Include letters
- `-s, --symbols`: Include special characters

## License
MIT License
