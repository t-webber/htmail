use crate::tools::html::GetElement;
use crate::windows::logger;
use wasm_bindgen::JsCast;

pub fn update_height(id: String) {
    let textarea = id.get_element().dyn_into::<web_sys::HtmlElement>().unwrap();

    if textarea
        .clone()
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap()
        .value()
        .is_empty()
    {
        match textarea.set_attribute("style", "height: 100%;") {
            Ok(()) => (),
            Err(err) => {
                logger::log(
                    &logger::WARNING,
                    &format!("Error on scrolling textarea: {err:?}"),
                );
            }
        }
        return;
    }

    // let scroll_height = textarea.scroll_height();
    // let client_height = textarea.client_height();

    let window = web_sys::window().expect("no global `window` exists");
    let html_element = textarea.clone().dyn_into::<web_sys::Element>().unwrap();
    let js_value = js_sys::Reflect::get(
        &window,
        &wasm_bindgen::JsValue::from_str("getComputedStyle"),
    )
    .unwrap();
    let get_computed_style: js_sys::Function = js_value.into();

    let styles = js_sys::Reflect::apply(
        &get_computed_style,
        &window,
        &js_sys::Array::of1(&html_element.into()),
    )
    .unwrap();

    let font_size: f32 =
        js_sys::Reflect::get(&styles, &wasm_bindgen::JsValue::from_str("fontSize"))
            .unwrap()
            .as_string()
            .unwrap()
            .trim_end_matches("px")
            .parse()
            .unwrap();

    // styles

    if let Ok(html_element) = textarea.dyn_into::<web_sys::HtmlElement>() {
        match html_element.set_attribute("style", "height: 0px") {
            Ok(()) => (),
            Err(err) => {
                logger::log(
                    &logger::WARNING,
                    &format!("Error on scrolling textarea: {err:?}"),
                );
            }
        }

        let scroll_height = html_element.scroll_height() as f32;

        match html_element
            .set_attribute("style", &format!("height: {}px", scroll_height + font_size))
        {
            Ok(()) => (),
            Err(err) => {
                logger::log(
                    &logger::WARNING,
                    &format!("Error on scrolling textarea: {err:?}"),
                );
            }
        }
    };
}

#[derive(yew::Properties, PartialEq, Default)]
pub struct ResponsiveTextareaProps {
    pub name: String,
    pub id: String,
    pub placeholder: String,
    pub oninput: Option<yew::Callback<yew::InputEvent>>,
}

#[yew::function_component(ResponsiveTextarea)]
pub fn textarea(props: &ResponsiveTextareaProps) -> yew::Html {
    let name = props.name.clone();
    let id = props.id.clone();
    let idclone = props.id.clone();
    let other_placeholder = props.placeholder.clone();
    let cloned = props
        .oninput
        .clone()
        .unwrap_or(yew::Callback::from(|_: yew::InputEvent| ()));
    let oninput = yew::Callback::from(move |event: yew::InputEvent| {
        cloned.emit(event.clone());
        update_height(idclone.clone());
    });
    yew::html!(
        <textarea name={name.clone()} id={id.clone()} placeholder={other_placeholder.clone()}
        oninput={oninput} />
    )
}
