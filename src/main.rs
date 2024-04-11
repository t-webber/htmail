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
#![allow(clippy::float_arithmetic)]
// BAD
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::impl_trait_in_params)]
// IDIOMATIC
#![allow(clippy::absolute_paths)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::mod_module_files)]
#![allow(clippy::needless_for_each)]

mod components;
mod plugins;
mod tools;
mod windows;

use components::body;
use components::fields::Fields;
use components::footer::Footer;
use components::titlebar::Titlebar;
use tools::fetch;
use windows::{fieldpopups, logger};

///[App]
struct App {
    from_drop_selection: fieldpopups::SelectionVec,
    to_drop_selection: fieldpopups::SelectionVec,
}

struct AppUpdateComplete {
    species: fieldpopups::SelectionSpecies,
    vec: fieldpopups::SelectionVec,
}

enum AppMsg {
    Update(fieldpopups::SelectionSpecies),
    UpdateComplete(AppUpdateComplete),
}

impl yew::Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            from_drop_selection: fieldpopups::SelectionVec::default(),
            to_drop_selection: fieldpopups::SelectionVec::default(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        yew::html!(
                <div class="app-container">
                    <div class="vbar">
                        <Titlebar />
                        <Fields msg_parent={ctx.link().callback(|content| AppMsg::Update(content))} />
                    </div>
                    <div class="grow">
                        <body::Body ..body::BodyProps::default() />
                    </div>
                    <div class="vbar">
                        <Footer />
                    </div>
                    <logger::Logger />
                    <fieldpopups::Windows from_drop_selection={self.from_drop_selection.clone()} to_drop_selection={self.to_drop_selection.clone()} />
                </div>
        )
    }

    fn update(&mut self, ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Update(species) => match species {
                fieldpopups::SelectionSpecies::None => return false,
                fieldpopups::SelectionSpecies::Profile => {
                    ctx.link().send_future(async move {
                        let profiles = fetch::get_profiles().await;
                        AppMsg::UpdateComplete(AppUpdateComplete {
                            species: species.clone(),
                            vec: profiles,
                        })
                    });
                }
                fieldpopups::SelectionSpecies::Recipient => {
                    ctx.link().send_future(async move {
                        let profiles = fetch::get_recipients().await;
                        AppMsg::UpdateComplete(AppUpdateComplete {
                            species: species.clone(),
                            vec: profiles,
                        })
                    });
                }
            },
            AppMsg::UpdateComplete(AppUpdateComplete { species, mut vec }) => match species {
                fieldpopups::SelectionSpecies::Profile => self.from_drop_selection = vec,

                fieldpopups::SelectionSpecies::Recipient => {
                    self.to_drop_selection = std::mem::take(&mut vec);
                }
                fieldpopups::SelectionSpecies::None => {}
            },
        }
        true
    }
}

// impl App {
//     async fn update_selection(
//         species: &fieldpopups::SelectionSpecies,
//     ) -> (
//         Option<Vec<fieldpopups::Selection>>,
//         Option<Vec<fieldpopups::Selection>>,
//     ) {
//         logger::log(&logger::SUCCESS, &format!("{:?}", species));
//         match species {
//             fieldpopups::SelectionSpecies::None => (None, None),
//             fieldpopups::SelectionSpecies::Profile => {
//                 let profiles = fetch::get_profiles().await;
//                 (Some(profiles), None)
//             }
//             fieldpopups::SelectionSpecies::Recipient => {
//                 let recipients = fetch::get_recipients().await;
//                 (None, Some(recipients))
//             }
//         }
//     }
// }

fn main() {
    yew::Renderer::<App>::new().render();
}
