use crate::tools::html::{self, GetElement};
use crate::windows::logger;
use wasm_bindgen::JsCast;

#[allow(clippy::expect_used)]
fn update_height(id: &str) {
    let textarea = id.get_element_cast::<web_sys::HtmlElement>();

    let window = html::get_window();
    let textarea_elt = textarea
        .clone()
        .dyn_into::<web_sys::Element>()
        .expect("Could not cast textarea into element");
    let js_value = js_sys::Reflect::get(
        &window,
        &wasm_bindgen::JsValue::from_str("getComputedStyle"),
    )
    .unwrap();
    let get_computed_style: js_sys::Function = js_value.into();

    let styles = js_sys::Reflect::apply(
        &get_computed_style,
        &window,
        &js_sys::Array::of1(&textarea_elt.into()),
    )
    .unwrap();

    let font_size: f64 =
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

        let scroll_height = f64::from(html_element.scroll_height());

        match html_element.set_attribute(
            "style",
            &format!(
                "height: {}px",
                scroll_height + font_size // .unwrap_or(scroll_height)
            ),
        ) {
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

#[derive(Default, PartialEq, yew::Properties)]
pub struct ResponsiveTextareaProps {
    pub name: String,
    pub id: String,
    pub value: String,
    pub placeholder: String,
    pub oninput: Option<yew::Callback<yew::InputEvent>>,
    pub onkeydown: Option<yew::Callback<yew::KeyboardEvent>>,
}

#[yew::function_component(ResponsiveTextarea)]
pub fn textarea(props: &ResponsiveTextareaProps) -> yew::Html {
    let ResponsiveTextareaProps {
        name,
        id,
        value,
        placeholder,
        oninput,
        onkeydown,
    } = props;
    let idclone = props.id.clone();
    let idclone2 = props.id.clone();
    let oninputcloned = oninput
        .clone()
        .unwrap_or_else(|| yew::Callback::from(|_: yew::InputEvent| ()));
    let onkeydowncloned = onkeydown
        .clone()
        .unwrap_or_else(|| yew::Callback::from(|_: yew::KeyboardEvent| ()));
    let on_input_callback = yew::Callback::from(move |event: yew::InputEvent| {
        oninputcloned.emit(event);
        update_height(&idclone.clone());
    });
    let on_key_down_callback = yew::Callback::from(move |event: yew::KeyboardEvent| {
        onkeydowncloned.emit(event);
        update_height(&idclone2.clone());
    });
    let readonly = id == "from-field";
    yew::html!(
        <textarea name={name.clone()} id={id.clone()} value={value.clone()} placeholder={placeholder.clone()}
        oninput={on_input_callback} onkeydown={on_key_down_callback} readonly={readonly}/>
    )
}
#[yew::function_component(Input)]
pub fn input(props: &ResponsiveTextareaProps) -> yew::Html {
    let ResponsiveTextareaProps {
        name,
        id,
        placeholder,
        oninput,
        onkeydown,
        ..
    } = props;
    let oninputcloned = oninput
        .clone()
        .unwrap_or_else(|| yew::Callback::from(|_: yew::InputEvent| ()));
    let onkeydowncloned = onkeydown
        .clone()
        .unwrap_or_else(|| yew::Callback::from(|_: yew::KeyboardEvent| ()));
    let readonly = id == "from-field";
    yew::html!(
        <input name={name.clone()} id={id.clone()} placeholder={placeholder.clone()}
        oninput={oninputcloned} onkeydown={onkeydowncloned} readonly={readonly}/>
    )
}
