use crate::tools::html::GetElement;
use wasm_bindgen::JsCast;

pub fn upload_n_write(input_id: String, render_id: String) {
    let input_elt: web_sys::Element = input_id.get_element();
    let render_elt: web_sys::Element = render_id.get_element();

    let content = input_elt
        .clone()
        .dyn_into::<web_sys::HtmlInputElement>()
        .map_or_else(
            |_| {
                input_elt
                    .dyn_into::<web_sys::HtmlTextAreaElement>()
                    .ok()
                    .map(|html| html.value())
            },
            |input| Some(input.value()),
        )
        .unwrap_or_default()
        .replace("\n\n", "<br>");

    render_elt
        .dyn_into::<web_sys::HtmlElement>()
        .ok()
        .map(|html| {
            html.set_inner_html(&content);
        })
        .unwrap();
    // .unwrap_or_else(|| logger::log("failure", "Error updating render element"));
}
