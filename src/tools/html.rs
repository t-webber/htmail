use wasm_bindgen::JsCast;

pub trait GetElement {
    fn get_element(self) -> web_sys::Element;
    fn get_element_cast<T: wasm_bindgen::JsCast>(self) -> T;
}

#[allow(clippy::panic)]
impl GetElement for &str {
    fn get_element(self) -> web_sys::Element {
        get_document()
            .get_element_by_id(self)
            .unwrap_or_else(|| panic!("No element with id = \"{self}\""))
    }

    fn get_element_cast<T: wasm_bindgen::JsCast>(self) -> T {
        self.get_element()
            .dyn_into::<T>()
            .unwrap_or_else(|err| panic!("Failed to cast element of id = {self}!\nError = {err:?}"))
    }
}

// pub trait SetStyle {
//     fn style(self, name: &str, value: &str) -> Result<web_sys::Element, web_sys::Element>;
//     fn end(self);
// }
// impl SetStyle for web_sys::Element {
//     fn style(self, name: &str, value: &str) -> Result<Self, Self> {
//         let style = js_sys::Reflect::get(&self, &wasm_bindgen::JsValue::from_str("style")).unwrap();
//         if js_sys::Reflect::set(
//             &style,
//             &wasm_bindgen::JsValue::from_str(name),
//             &wasm_bindgen::JsValue::from_str(value),
//         )
//         .unwrap()
//         {
//             Ok(self)
//         } else {
//             Err(self)
//         }
//     }

//     fn end(self) {
//         ()
//     }
// }
// impl SetStyle for Result<web_sys::Element, web_sys::Element> {
//     fn style(self, name: &str, value: &str) -> Self {
//         let (success, inner) = match self {
//             Ok(inner) => (true, inner),
//             Err(inner) => (false, inner),
//         };
//         let style =
//             js_sys::Reflect::get(&inner, &wasm_bindgen::JsValue::from_str("style")).unwrap();
//         if js_sys::Reflect::set(
//             &style,
//             &wasm_bindgen::JsValue::from_str(name),
//             &wasm_bindgen::JsValue::from_str(value),
//         )
//         .unwrap()
//             && success
//         {
//             Ok(inner)
//         } else {
//             Err(inner)
//         }
//     }

//     fn end(self) {
//         ()
//     }
// }

#[allow(clippy::expect_used)]
pub fn get_window() -> web_sys::Window {
    web_sys::window().expect("No window found")
}

#[allow(clippy::expect_used)]
pub fn get_document() -> web_sys::Document {
    get_window().document().expect("No document found")
}
