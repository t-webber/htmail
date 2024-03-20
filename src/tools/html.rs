pub trait GetElement {
    fn get_element(self) -> web_sys::Element;
}

impl GetElement for &str {
    fn get_element(self) -> web_sys::Element {
        web_sys::window()
            .expect("No window found")
            .document()
            .expect("No document found")
            .get_element_by_id(self)
            .expect("No element with this id")
    }
}
