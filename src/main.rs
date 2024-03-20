#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]

mod components;
mod plugins;
mod tools;
mod windows;

use components::body::Body;
use components::fields::Fields;
use components::titlebar::Titlebar;
use windows::fieldpopups::Windows;
use windows::logger::Logger;

#[yew::function_component(App)]
fn app() -> yew::Html {
    yew::html!(
        <div class="app-container">
            <Titlebar />
            <Fields />
            <Windows />
            <Body />
            <Logger />
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
