use crate::components::{
    body::{Container, ContainerProps},
    fields::FieldSet,
    footer::Footer,
    titlebar::Titlebar,
};
use crate::tools::{
    fetch::{get_profiles, get_recipients},
    routes::Route,
};
use crate::windows::{
    fieldpopups::{SelectionSpecies, SelectionVec, Windows},
    logger::Logger,
};

use yew_router::prelude::*;

pub struct App {
    from_drop_selection: SelectionVec,
    to_drop_selection: SelectionVec,
}

pub struct AppUpdateComplete {
    species: SelectionSpecies,
    vec: SelectionVec,
}

pub enum AppMsg {
    Update(SelectionSpecies),
    UpdateComplete(AppUpdateComplete),
}

impl yew::Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            from_drop_selection: SelectionVec::default(),
            to_drop_selection: SelectionVec::default(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let navigator = ctx.link().navigator().unwrap();
        let onclick: yew::Callback<web_sys::MouseEvent> =
            yew::Callback::from(move |_: web_sys::MouseEvent| {
                navigator.push(&Route::Home);
            });
        yew::html!(
                <div class="app-container">
                <button {onclick}>{"go home"}</button>
                    <div class="vbar">
                        <Titlebar />
                        <FieldSet msg_parent={ctx.link().callback(AppMsg::Update)} />
                    </div>
                    <div class="grow">
                        <Container  ..ContainerProps::default() />
                    </div>
                    <div class="vbar">
                        <Footer />
                    </div>
                    <Logger />
                    <Windows from_drop_selection={self.from_drop_selection.clone()} to_drop_selection={self.to_drop_selection.clone()} />
                </div>
        )
    }

    fn update(&mut self, ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        #[allow(clippy::expect_used)]
        match msg {
            AppMsg::Update(species) => match species {
                SelectionSpecies::None => return false,
                SelectionSpecies::Profile => {
                    ctx.link().send_future(async move {
                        let profiles = get_profiles().await;
                        AppMsg::UpdateComplete(AppUpdateComplete {
                            species: species.clone(),
                            vec: profiles.expect("Failed to fetch profiles"),
                        })
                    });
                }
                SelectionSpecies::Recipient => {
                    ctx.link().send_future(async move {
                        let profiles = get_recipients().await;
                        AppMsg::UpdateComplete(AppUpdateComplete {
                            species: species.clone(),
                            vec: profiles.expect("Failed to fetch recipients"),
                        })
                    });
                }
            },
            AppMsg::UpdateComplete(AppUpdateComplete { species, mut vec }) => match species {
                SelectionSpecies::Profile => self.from_drop_selection = vec,

                SelectionSpecies::Recipient => {
                    self.to_drop_selection = core::mem::take(&mut vec);
                }
                SelectionSpecies::None => {}
            },
        }
        true
    }
}
