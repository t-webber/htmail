# HTMail: Easily send emails with HTML content

## Getting started

### Requirements

First of all, you will need to install `cargo` and `rust`. You can find everything you need on the [official website](https://www.rust-lang.org/learn/get-started).

The project needs the `wasm32-unknown-unknown` target to be installed. You can do so by running the following command:

```shell
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

This project uses `tauri-2.0.0-beta` and `yew-0.21` to build the application. To install `tauri`, run the following command:

```shell
cargo install create-tauri-app
cargo install tauri-cli --version "^2.0.0-beta"
cargo install --locked trunk
```

You can find everything you need on `Tauri` on the [official website](https://beta.tauri.app/fr/guides/).

### Run the project

To run the project, simply write `cargo tauri dev` in your terminal. If you want to run it on your smartphone, you will need to run `cargo tauri android dev` and follow the given instructions.

### Build the project

To build the project, simply write `cargo tauri build <architecture>` in your terminal.
