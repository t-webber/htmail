use crate::tools::fetch;

#[yew::function_component(Footer)]
pub fn footer() -> yew::Html {
    yew::html!(
        <div class="footer-component">
            <div class="left-empty">{" "}</div>
            <div class="copy">{"\u{00A9} 2024 HTMaiL - Tom Webber"}</div>
            <button class="send" onclick={|_| fetch::send_mail()}>{"Send"}</button>
        </div>
    )
}
