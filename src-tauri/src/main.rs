#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::implicit_return)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::question_mark_used)]
// BAD
#![allow(clippy::multiple_crate_versions)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

fn main() {
    htmail_lib::run();
    // mail::main();
}
