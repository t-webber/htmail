#[yew::function_component(Titlebar)]
pub fn titlebar() -> yew::Html {
    yew::html!(
        <div class="titlebar-container">
            <div class="title">{"HTMail"}</div>
            <div class="menu">
                <div class="dash"></div>
                <div class="dash"></div>
                <div class="dash"></div>
            </div>
        </div>
    )
}
