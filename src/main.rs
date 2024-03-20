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
use components::footer::Footer;
use components::titlebar::Titlebar;
use windows::fieldpopups::Windows;
use windows::logger::Logger;

#[yew::function_component(App)]
fn app() -> yew::Html {
    yew::html!(
        <div class="app-container" style="height: 100vh; display: flex; flex-direction: column;">
            <div style="display: flex; flex-direction: column;">
                <Titlebar />
                <Fields />
            </div>
            <div style="display: flex; flex-grow: 1;">
                <Body />
            </div>
            <div style="display: flex; flex-direction: column;">
                <Footer />
            </div>

            <Logger />
            <Windows />
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
