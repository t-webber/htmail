use crate::plugins::textarea;
use crate::tools::body as btools;

#[yew::function_component(BodyArea)]
fn body_area() -> yew::Html {
    let id = "body-textarea-textarea";

    yew::html!(
            <div class="body-textarea">

            <textarea::ResponsiveTextarea name="bodyarea" id={id} placeholder="Enter email body here..." oninput={Some(
                yew::Callback::<yew::InputEvent>::from(move |_: yew::InputEvent| {
                        // btools::update_height(id.to_owned());
                        btools::upload_n_write(id.to_owned(), "body-render".to_owned());
    }))} />
            </div>
        )
}

#[yew::function_component(Body)]
pub fn body() -> yew::Html {
    yew::html!(
        <div class="body-container">
            <BodyArea />
        <div id="body-render" class="body-render"></div>
        </div>
    )
}
