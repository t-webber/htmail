#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
// GOOD
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::implicit_return)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::question_mark_used)]
// BAD
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::impl_trait_in_params)]
#![allow(clippy::pattern_type_mismatch)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::as_conversions)]
#![allow(clippy::field_reassign_with_default)]
// IDIOMATIC
#![allow(clippy::absolute_paths)]
#![allow(clippy::mod_module_files)]
#![allow(clippy::needless_for_each)]
#![allow(clippy::missing_trait_methods)]

mod components;
mod pages;
mod plugins;
mod tools;
mod windows;

use tools::routes::RoutedApp;

fn main() {
    yew::Renderer::<RoutedApp>::new().render();
}
