# HTMail: Easily send emails with HTML content

## Getting started

### Installation

First of all, you will need to install `cargo` and `rust`. You can find everything you need on the [official website](https://www.rust-lang.org/learn/get-started).

Then, you will need to run the following command: `rustup target add wasm32-unknown-unknown` to compile your rust code to wasm.

This project uses `tauri-2-beta` and `yew-0.21` to build the application. To install `tauri`, you will find everything you need on the [official website](https://beta.tauri.app/fr/guides/).

### Run the project

To run the project, simply write `cargo tauri dev` in your terminal. If you want to run it on your smartphone, you will need to run `cargo tauri android dev`, and follow the given instructions.

### Build the project

To build the project, simply write `cargo tauri build <architecture>` in your terminal.
