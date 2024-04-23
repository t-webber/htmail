use crate::pages::send::App as SendApp;

#[derive(Clone, yew_router::Routable, PartialEq)]
pub enum Route {
    #[not_found]
    #[at("/")]
    Home,
    #[at("/send")]
    Send,
}

///[Home]
#[derive(PartialEq, yew::Properties)]
pub struct HomeProps {}

#[yew::function_component(Home)]
pub fn home(props: &HomeProps) -> yew::Html {
    let navigator = yew_router::prelude::use_navigator().unwrap();
    let onclick = yew::Callback::from(move |_| {
        navigator.push(&Route::Send);
    });
    let HomeProps {} = props;
    yew::html! {
        <div>
            <button {onclick}>{ "click here to go to sender" }</button>
        </div>
    }
}

fn render(route: Route) -> yew::Html {
    match route {
        Route::Send => yew::html! {
            <SendApp />
        },
        Route::Home => {
            yew::html! { <Home /> }
        }
    }
}

#[yew::function_component(RoutedApp)]
pub fn routed_app() -> yew::Html {
    yew::html! {
        <yew_router::BrowserRouter>
            <yew_router::Switch<Route> {render} />
        </yew_router::BrowserRouter>
    }
}
