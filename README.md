# Aeon desktop application
This is an implementation of [Aeon](http://biodivine.fi.muni.cz/aeon) tool as desktop application.

### Prerequisites
- [Rust](https://www.rust-lang.org/) (version 1.60)
- [npm](https://nodejs.org/)

### Build
Once you install Rust, you can build the app from source.
Simply run `cargo build --release` in the `src-tauri` directory. The binary will be located in `src-tauri/target/release/biodivine-aeon-desktop`.

### Installation
Installation files are located in the last [release](https://github.com/sybila/biodivine-aeon-desktop/releases).
Supported OS systems are Windows, macOS and Linux.

### App launch
Run `npm run start`

### Scripts
- Run ```npm run js-code-check```: check formatting of JavaScript code based on the eslint rules located in `.eslinrc.json` file.

- Run ```npm run js-code-format```: format JavaScript code based on the eslint rules located in `.eslinrc.json` file.

- Run ```npm run rust-code-check```: analyze source code for potential errors, style issues, and other code quality concerns.

- Run ```npm run rust-code-format```: format and enforce a consistent and standardized coding style across a Rust project


